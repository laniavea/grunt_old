use std::time::{Instant, Duration};

use grunt::types::*;
use grunt::model3d::generate_model;

pub fn bench(runs_difficulty: usize, run_tries: usize) -> (Duration, Duration, Duration) {
    println!("Export Model&Mask benchmark...");
    let axis_size = [100., 200., 500., 4000.]; let border_deviation = [5., 10.];
    let max_step = [Some(1), Some(2), Some(3), None];
    let layers_dist = [vec![20, 30, 20], vec![1000, 200, 100], vec![300, 400, 50, 100]];
    let fill_preset = vec![vec![10, 20], vec![100, 200], vec![300, 300], vec![400, 800]];

    let save_state_model = vec!["model"];
    let save_state_mask = vec!["model_mask"];

    let mut diff_params: Vec<generation_params::Params3D> = Vec::new();
    let ca_exp: Vec<AxisExportType> = (0..3).map(|_| AxisExportType::CustomAxis(vec![1.0])).collect();

    for run in 0..runs_difficulty {
        let now_axis_size = axis_size[run % axis_size.len()];
        let now_border_deviation = border_deviation[run % border_deviation.len()];
        let now_max_step = max_step[run % max_step.len()];
        let now_layers_dist = layers_dist[run % layers_dist.len()].clone();

        let mut params = generation_params::Params3D::new();

        params.set_x_axis(Axis::generate_axis(0.0, now_axis_size, None).unwrap());
        params.set_y_axis(Axis::generate_axis(0.0, now_axis_size, None).unwrap());

        params.set_layers_dist(LayersDist::create_from_vec(now_layers_dist).unwrap_or_default());

        let mut borders = LayersBorder::new();
        borders.set_border_deviation(now_border_deviation).unwrap();
        borders.set_border_max_step(now_max_step);

        params.set_layers_border(borders);

        let mut fill = LayersFill::new();
        fill.set_values_preset(fill_preset.clone()).unwrap();

        params.set_layers_fill(fill);

        params.set_mask_needed(true);
        params.set_model_needed(true);

        diff_params.push(params)
    }

    let mut full_elapsed: Duration = Duration::default();
    let mut full_elapsed_model: Duration = Duration::default();
    let mut full_elapsed_mask: Duration = Duration::default();

    for run_try in 0..run_tries {
        println!("\tRun №{}", run_try+1);
        let mut all_elapsed: Duration = Duration::default();
        let mut all_elapsed_model: Duration = Duration::default();
        let mut all_elapsed_mask: Duration = Duration::default();

        for run in 0..runs_difficulty {
            let now_axis_size = axis_size[run % axis_size.len()];
            let now_model_size = now_axis_size * now_axis_size * diff_params[run].layers_dist().get_layers_sum() as f32;

            print!("\t\tGenerating tests...");
            let model = generate_model(diff_params[run].clone()).unwrap();
            println!(" Completed!");

            let now_test = Instant::now();

            println!("\t\tTest №{}, Number of axes: {}, Nubmer of elements in model {}", 
                run+1, now_axis_size, now_model_size);

            let now_test_model = Instant::now();
            model.export_model("TestModelBench.test.bench", &save_state_model, &ca_exp).unwrap();
            let now_test_model = now_test_model.elapsed();

            let now_test_mask = Instant::now();
            model.export_model("TestModelBench.test.bench", &save_state_mask, &ca_exp).unwrap();
            let now_test_mask = now_test_mask.elapsed();

            let elapsed_test = now_test.elapsed();

            all_elapsed += elapsed_test;
            all_elapsed_model += now_test_model;
            all_elapsed_mask += now_test_mask;

            println!("\t\tTest elapsed: {:.2?}, Model export: {:.2?}, Mask export: {:.2?}",
                elapsed_test, now_test_model, now_test_mask);
        }
        full_elapsed += all_elapsed;
        full_elapsed_model += all_elapsed_model;
        full_elapsed_mask += all_elapsed_mask;

        println!("\tRun elapsed: {:.2?}, Model export: {:.2?}, Mask export: {:.2?}",
            all_elapsed, all_elapsed_model, all_elapsed_mask);
    }

    full_elapsed /= run_tries as u32;
    full_elapsed_model /= run_tries as u32;
    full_elapsed_mask /= run_tries as u32;

    println!("Export Model&Mask benchmark completed succesfully");

    (full_elapsed, full_elapsed_model, full_elapsed_mask)
}
