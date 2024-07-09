#[cfg(debug_assertions)]
use log::{trace, info, error};

pub mod random_border;

use crate::types::generation_params::Params3D;

pub fn create_layers_borders_3d(params: &Params3D) -> Result<Vec<Vec<Vec<i32>>>, &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting creating 3D borders");

    let layers_dist_params = params.layers_dist();
    let layers_borders_params = params.layers_border();

    let borders_deviation = layers_borders_params.border_deviation();
    let deviation_override = layers_borders_params.deviation_override();
    let layers_count = layers_dist_params.get_layers_count();
    let x_size = params.x_axis().blocks_count();
    let y_size = params.y_axis().blocks_count();

    #[cfg(debug_assertions)]
    trace!("Generating with this params: layers_count-{layers_count}, x_size-{x_size}, y_size-{y_size}");

    let mut layers_borders = vec![vec![vec![0i32; x_size]; y_size]; layers_count];

    let mut upper_limit: i32;
    let mut lower_limit: i32;
    let max_step = params.layers_border().border_max_step();

    for (i, layer) in layers_borders.iter_mut().enumerate() {
        // Generating limits of generation (max and min heights of layer)
        if let Some(deviation_override) = deviation_override.as_ref() {
            let deviation_override_len = deviation_override.len();

            lower_limit = deviation_override[i % deviation_override_len][0];
            upper_limit = deviation_override[i % deviation_override_len][1];
        } else {
            let default_value = layers_dist_params.get_layers_dist_summed()[i];

            upper_limit = if borders_deviation >= 1.0 {
                default_value + borders_deviation as i32 
            } else {
                let model_size_value = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);
                default_value + (borders_deviation * model_size_value as f32) as i32 
            };

            lower_limit = if borders_deviation >= 1.0 {
                default_value - (borders_deviation as i32)
            } else {
                let model_size_value = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);
                default_value - (borders_deviation * model_size_value as f32) as i32 
            };
        }

        if lower_limit < 0 { lower_limit = 0; }

        match params.layers_border().border_type().as_str() {
            "random" => random_border::random_layer_creation(max_step, upper_limit, lower_limit, layer)?,
            _ => return Err("Incorrect border type"),
        };

        #[cfg(debug_assertions)]
        if let Err(err) = validate_layer(max_step, upper_limit, lower_limit, layer, i) {
            error!("Validating for layer {i} Failed: {err}");
            return Err("Model is not valid");
        }
        #[cfg(debug_assertions)]
        trace!("Validating for layer {i} completed succesfully");
    }

    if params.layers_border().border_mod_func().is_some() {
        #[cfg(debug_assertions)]
        trace!("Border's mod function found");

        let mod_func = params.layers_border().border_mod_func().unwrap();
        for (layer_num, layer) in layers_borders.iter_mut().enumerate() {
            for (y_num, layer_y) in layer.iter_mut().enumerate() {
                for (x_num, layer_x) in layer_y.iter_mut().enumerate() {
                    *layer_x -= mod_func(x_num, y_num, layer_num, *layer_x)
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    trace!("3D borders were generated succesfully");

	Ok(layers_borders)
}

pub fn validate_layer(
    max_step: Option<i32>,
    upper_limit: i32,
    lower_limit: i32,
    layer: &[Vec<i32>],
    now_layer_id: usize)
-> Result<(), &'static str> {
    #[cfg(debug_assertions)]
    trace!("Layer's params: max_step-{:?}, upper_limit-{upper_limit}, lower_limit-{lower_limit}", max_step);

    let mut err_elems = 0;

    for i in 0..layer.len() {
        for j in 0..layer[i].len() {
            if !(lower_limit <= layer[i][j] && layer[i][j] <= upper_limit) {
                #[cfg(debug_assertions)]
                info!("In layer {now_layer_id} elem {i}, {j} cannot be validated");
                err_elems += 1;
            } else if max_step.is_some(){
                let max_step = max_step.unwrap();

                if i != layer.len() - 1 && (layer[i][j] - layer[i+1][j]).abs() > max_step {
                    #[cfg(debug_assertions)]
                    info!("In layer {now_layer_id} elem {i}, {j} cannot be validated");
                    err_elems += 1;
                    continue;
                }

                if j != layer[i].len() - 1 && layer[i][j] - layer[i][j+1].abs() > max_step{
                        #[cfg(debug_assertions)]
                        info!("In layer {now_layer_id} elem {i}, {j} cannot be validated");
                        err_elems += 1;
                }
            }
        }
    }

    if err_elems != 0 {
        #[cfg(debug_assertions)]
        error!("There are {err_elems} wrong generated elements in {now_layer_id} layer");
        return Err("Errors while model validatiion");
    }

    Ok(())
}
