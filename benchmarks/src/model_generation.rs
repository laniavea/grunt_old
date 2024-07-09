use std::time::{Instant, Duration};

use grunt::types::*;
use grunt::model3d::generate_model;

pub fn bench(runs_difficulty: usize, run_tries: usize) -> (Duration, Duration, Duration, Duration) {
    println!("Model generation benchmark...");

    let axis_size = [10., 100., 100., 500.];
    let border_deviation = [5., 10.];
    let max_step = [Some(3), Some(5)];
    let layers_dist = [vec![20, 30, 20], vec![200, 300, 200], vec![40, 60, 80, 3000, 21000], vec![2000, 3000, 2000]];
    let fill_preset = vec![vec![10, 20], vec![100, 200], vec![300, 300], vec![400, 800]];

    let mut diff_params: Vec<generation_params::Params3D> = Vec::new();

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
    let mut full_elapsed_full: Duration = Duration::default();
    let mut full_elapsed_mask: Duration = Duration::default();
    let mut full_elapsed_model: Duration = Duration::default();

    for run_try in 0..run_tries {
        println!("\tRun №{}", run_try+1);

        let mut all_elapsed: Duration = Duration::default();
        let mut all_elapsed_full: Duration = Duration::default();
        let mut all_elapsed_mask: Duration = Duration::default();
        let mut all_elapsed_model: Duration = Duration::default();

        let now = Instant::now();

        for run in 0..runs_difficulty {
            let now_params = diff_params[run % diff_params.len()].clone();
            let mut now_params_mask = diff_params[run % diff_params.len()].clone();
            now_params_mask.set_model_needed(false);
            let mut now_params_model = diff_params[run % diff_params.len()].clone();
            now_params_model.set_mask_needed(false);

            println!("\t\tTest №{}", run+1);

            let now_test = Instant::now();

            let now_test_model_full = Instant::now();
            let _ = generate_model(now_params);
            let now_test_model_full = now_test_model_full.elapsed();

            let now_test_model_mask = Instant::now();
            let _ = generate_model(now_params_mask);
            let now_test_model_mask = now_test_model_mask.elapsed();

            let now_test_model_model = Instant::now();
            let _ = generate_model(now_params_model);
            let now_test_model_model = now_test_model_model.elapsed();

            let elapsed_test = now_test.elapsed();

            all_elapsed += elapsed_test;
            all_elapsed_full += now_test_model_full;
            all_elapsed_mask += now_test_model_mask;
            all_elapsed_model += now_test_model_model;

            println!("\t\tTest elapsed: {:.2?}, Full model: {:.2?}, Mask only: {:.2?}, Model only: {:.2?}", 
                elapsed_test, now_test_model_full, now_test_model_mask, now_test_model_model)
        }
        full_elapsed += all_elapsed;
        full_elapsed_full += all_elapsed_full;
        full_elapsed_mask += all_elapsed_mask;
        full_elapsed_model += all_elapsed_model;

        let elapsed = now.elapsed();
        println!("\tRun elapsed: {:.2?}, Full model: {:.2?}, Mask only: {:.2?}, Model only: {:.2?}",
            elapsed, all_elapsed_full, all_elapsed_mask, all_elapsed_model);
    }

    full_elapsed /= run_tries as u32;
    full_elapsed_full /= run_tries as u32;
    full_elapsed_mask /= run_tries as u32;
    full_elapsed_model /= run_tries as u32;

    println!("Model generation benchmark completed succesfully");

    (full_elapsed, full_elapsed_full, full_elapsed_mask, full_elapsed_model)
}
