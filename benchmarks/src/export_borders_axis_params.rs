use std::time::{Instant, Duration};

use grunt::types::*;
use grunt::model3d::generate_model;

pub fn bench(runs_difficulty: usize, run_tries: usize) -> (Duration, Duration, Duration) {
    println!("Export Borders&Axis&Params benchmark...");
    let axis_size = [100., 2000., 5000., 7500., 10000., 15000.];
    let border_deviation = [5., 10.];
    let max_step = [Some(1), Some(2), Some(3), Some(4), Some(5), None];
    let layers_dist = [vec![20, 30, 20], vec![200, 300, 200], vec![40, 60, 80, 3000, 2000], vec![2000, 3000, 2000]];

    let save_state = vec!["params", "borders"];

    let mut diff_params: Vec<generation_params::Params3D> = Vec::new();
    let num_exp: Vec<AxisExportType> = (0..3).map(|_| AxisExportType::AsNum).collect();
    let scale_exp: Vec<AxisExportType> = 
        vec![AxisExportType::AsSelf, AxisExportType::Scale(2.5), AxisExportType::Scale(0.5)];

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

        params.set_mask_needed(false);
        params.set_model_needed(false);

        diff_params.push(params)
    }

    let mut full_elapsed: Duration = Duration::default();
    let mut full_elapsed_num: Duration = Duration::default();
    let mut full_elapsed_scale: Duration = Duration::default();

    for run_try in 0..run_tries {
        println!("\tRun №{}", run_try+1);
        let mut all_elapsed: Duration = Duration::default();
        let mut all_elapsed_num: Duration = Duration::default();
        let mut all_elapsed_scale: Duration = Duration::default();

        for run in 0..runs_difficulty {
            let now_axis_size = axis_size[run % axis_size.len()];
            let now_borders_count = now_axis_size * now_axis_size * layers_dist[run % layers_dist.len()].len() as f32;

            print!("\t\tGenerating tests...");
            let model = generate_model(diff_params[run].clone()).unwrap();
            println!(" Completed!");

            let now_test = Instant::now();

            println!("\t\tTest №{}, Number of axes: {}, Nubmer of borders {}", run+1, now_axis_size, now_borders_count);

            let now_test_num = Instant::now();
            model.export_model("TestModelBench.test.bench", &save_state, &num_exp).unwrap();
            let now_test_num = now_test_num.elapsed();

            let now_test_scale = Instant::now();
            model.export_model("TestModelBench.test.bench", &save_state, &scale_exp).unwrap();
            let now_test_scale = now_test_scale.elapsed();

            let elapsed_test = now_test.elapsed();

            all_elapsed += elapsed_test;
            all_elapsed_num += now_test_num;
            all_elapsed_scale += now_test_scale;

            println!("\t\tTest elapsed: {:.2?}, Num export: {:.2?}, Scale export: {:.2?}",
                elapsed_test, now_test_num, now_test_scale);
        }
        full_elapsed += all_elapsed;
        full_elapsed_num += all_elapsed_num;
        full_elapsed_scale += all_elapsed_scale;

        println!("\tRun elapsed: {:.2?}, Num export: {:.2?}, Scale export: {:.2?}",
            all_elapsed, all_elapsed_num, all_elapsed_scale);
    }

    full_elapsed /= run_tries as u32;
    full_elapsed_num /= run_tries as u32;
    full_elapsed_scale /= run_tries as u32;

    println!("Export Borders&Axis&Params benchmark completed succesfully");

    (full_elapsed, full_elapsed_num, full_elapsed_scale)
}
