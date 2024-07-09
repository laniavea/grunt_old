use std::time::{Instant, Duration};

use grunt::types::*;
use grunt::model3d::generate_model;

pub fn bench(runs_difficulty: usize, run_tries: usize) -> Duration {
    println!("Random border benchmark...");
    let axis_size = [10., 100., 500., 1000., 2000., 5000., 7500., 10000., 15000., 20000.];
    let border_deviation = [5., 10.];
    let max_step = [Some(1), Some(2), Some(3), Some(4), Some(5), None];

    let mut all_elapsed: Duration = Duration::default();

    for run_try in 0..run_tries {
        println!("\tRun №{}", run_try+1);
        let now = Instant::now();

        for run in 0..runs_difficulty {
            let now_test = Instant::now();

            let now_axis_size = axis_size[run % axis_size.len()];
            let now_border_deviation = border_deviation[run % border_deviation.len()];
            let now_max_step = max_step[run % max_step.len()];

            println!("\t\tTest №{}, axis: {now_axis_size}, border_deviation: {now_border_deviation}, max_step: {:?}",
                run+1, now_max_step);

            let mut params = generation_params::Params3D::new();

            params.set_x_axis(Axis::generate_axis(0.0, now_axis_size, None).unwrap());
            params.set_y_axis(Axis::generate_axis(0.0, now_axis_size, None).unwrap());

            params.set_layers_dist(LayersDist::create_from_vec([20, 30, 20].to_vec()).unwrap_or_default());

            let mut borders = LayersBorder::new();
            borders.set_border_deviation(now_border_deviation).unwrap();
            borders.set_border_max_step(now_max_step);

            params.set_layers_border(borders);

            params.set_mask_needed(false);
            params.set_model_needed(false);

            let _ = generate_model(params).unwrap();

            let elapsed_test = now_test.elapsed();
            println!("\t\tTest elapsed: {:.2?}", elapsed_test)
        }

        let elapsed = now.elapsed();
        println!("\tRun elapsed: {:.2?}", elapsed);
        all_elapsed += elapsed;
    }
    println!("Random border benchmark completed succesfully");

    all_elapsed / run_tries as u32 
}
