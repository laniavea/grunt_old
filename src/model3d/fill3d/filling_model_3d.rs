#[cfg(debug_assertions)]
use log::trace;

use rand::distributions::Distribution;

use crate::model3d::fill3d::GenerationTypes;

type ModelAndMaskType = (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<u8>>>);

fn generate_consts(borders: &Vec<Vec<Vec<i32>>>) -> (i32, usize, usize, usize) {
    let mut max_elem = 0;
    for depth in borders {
        for y_cord in depth {
            for x_cord_value in y_cord {
                if *x_cord_value > max_elem {
                    max_elem = *x_cord_value;
                } 
            }
        } 
    }

    (max_elem, borders.len(), borders[0].len(), borders[0][0].len())
}

pub fn create_full_model_with_mask(
    borders: &Vec<Vec<Vec<i32>>>,
    fill_values: &[GenerationTypes]
) -> ModelAndMaskType {
    #[cfg(debug_assertions)]
    trace!("Starting filling model: model and mask");

    let (max_elem, layers_count, y_size, x_size) = generate_consts(borders);
    let max_elem: usize = max_elem.try_into().unwrap();

    let mut model: Vec<Vec<Vec<i32>>> = Vec::with_capacity(x_size);
    let mut model_mask: Vec<Vec<Vec<u8>>> = Vec::with_capacity(x_size);
    let mut rng = rand::thread_rng();

    for x_cord in 0..x_size {

        let mut now_x: Vec<Vec<i32>> = Vec::with_capacity(y_size);
        let mut now_x_mask: Vec<Vec<u8>> = Vec::with_capacity(y_size);

        for y_cord in 0..y_size {

            let mut now_y: Vec<i32> = Vec::with_capacity(max_elem);
            let mut now_y_mask: Vec<u8> = Vec::with_capacity(max_elem);

            let mut now_index: usize = 0;
            let mut now_index_u8: u8 = 0;
            let mut now_depth: usize = borders[now_index][y_cord][x_cord].try_into().unwrap_or(0);

            for depth in 0..max_elem {

                if depth >= now_depth && now_index < layers_count - 1 {
                    loop {
                        now_index += 1;
                        now_depth = borders[now_index][y_cord][x_cord].try_into().unwrap_or(0);
                        if now_depth > depth || now_index == layers_count - 1 {
                            now_index_u8 = now_index as u8;
                            break;
                        }
                    }
                }

                now_y.push(match fill_values[now_index] {
                    GenerationTypes::GenerationExact(value) => value,
                    GenerationTypes::GenerationRange(generation_range) => generation_range.sample(&mut rng)
                });
                now_y_mask.push(now_index_u8);
            }

            now_x.push(now_y);
            now_x_mask.push(now_y_mask);
        }

        model.push(now_x);
        model_mask.push(now_x_mask);
    }

    #[cfg(debug_assertions)]
    trace!("Model and mask were filled succesfully");

    (model, model_mask)
}

pub fn create_full_model_without_mask(
    borders: &Vec<Vec<Vec<i32>>>,
    fill_values: &[GenerationTypes]
) -> Vec<Vec<Vec<i32>>> {
    #[cfg(debug_assertions)]
    trace!("Starting filling only model");

    let (max_elem, layers_count, y_size, x_size) = generate_consts(borders);
    let max_elem: usize = max_elem.try_into().unwrap();

    let mut model: Vec<Vec<Vec<i32>>> = Vec::with_capacity(x_size);
    let mut rng = rand::thread_rng();

    for x_cord in 0..x_size {

        let mut now_x: Vec<Vec<i32>> = Vec::with_capacity(y_size);

        for y_cord in 0..y_size {

            let mut now_y: Vec<i32> = Vec::with_capacity(max_elem);

            let mut now_index: usize = 0;
            let mut now_depth: usize = borders[now_index][y_cord][x_cord].try_into().unwrap_or(0);

            for depth in 0..max_elem {

                if depth >= now_depth && now_index < layers_count - 1 {
                    loop {
                        now_index += 1;
                        now_depth = borders[now_index][y_cord][x_cord].try_into().unwrap_or(0);
                        if now_depth > depth || now_index == layers_count - 1 {
                            break;
                        }
                    }
                }

                now_y.push(match fill_values[now_index] {
                    GenerationTypes::GenerationExact(value) => value,
                    GenerationTypes::GenerationRange(generation_range) => generation_range.sample(&mut rng)
                });
            }

            now_x.push(now_y);
        }

        model.push(now_x);
    }

    #[cfg(debug_assertions)]
    trace!("Model was filled succesfully");

    model
}

pub fn create_only_mask(
    borders: &Vec<Vec<Vec<i32>>>,
) -> Vec<Vec<Vec<u8>>> {
    #[cfg(debug_assertions)]
    trace!("Starting filling only mask");

    let (max_elem, layers_count, y_size, x_size) = generate_consts(borders);
    let max_elem: usize = max_elem.try_into().unwrap();

    let mut model_mask: Vec<Vec<Vec<u8>>> = Vec::with_capacity(x_size);

    for x_cord in 0..x_size {

        let mut now_x_mask: Vec<Vec<u8>> = Vec::with_capacity(y_size);

        for y_cord in 0..y_size {

            let mut now_y_mask: Vec<u8> = Vec::with_capacity(max_elem);

            let mut now_index: usize = 0;
            let mut now_index_u8: u8 = 0;
            let mut now_depth: usize = borders[now_index][y_cord][x_cord].try_into().unwrap_or(0);

            for depth in 0..max_elem {

                if depth >= now_depth && now_index < layers_count - 1 {
                    loop {
                        now_index += 1;
                        now_depth = borders[now_index][y_cord][x_cord].try_into().unwrap_or(0);
                        if now_depth > depth || now_index == layers_count - 1 {
                            now_index_u8 = now_index as u8;
                            break;
                        }
                    }
                }

                now_y_mask.push(now_index_u8);
            }

            now_x_mask.push(now_y_mask);
        }

        model_mask.push(now_x_mask);
    }

    #[cfg(debug_assertions)]
    trace!("Mask was filled succesfully");

    model_mask
}
