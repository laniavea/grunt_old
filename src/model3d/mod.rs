#[cfg(debug_assertions)]
use log::trace;

use crate::types::generation_params::Params3D;

pub mod borders3d;
pub mod shifts3d;
pub mod fill3d;
pub mod export;
pub mod convert_data;

pub fn generate_model(params: Params3D) -> Result<Model3D, &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting generating 3D model");

    let mut borders = borders3d::create_layers_borders_3d(&params)?;
    let mut max_depth = get_max_depth(&borders);

    if !(params.shifts().is_empty()) {
        #[cfg(debug_assertions)]
        trace!("{} shifts found", params.shifts().len());

        for shift in params.shifts() {
            shifts3d::add_shift_3d::add_shift(&params, &mut borders, shift, &mut max_depth);
        }
    }

    let (model, model_mask, fill_values) = if params.model_needed() || params.mask_needed() {
        fill3d::fill(&params, &borders)
    } else {
        (Vec::new(), Vec::new(), Vec::new())
    };

    let final_model = Model3D::new(model, model_mask, borders, fill_values, max_depth, params);

    Ok(final_model)
}

#[derive(Debug, Clone)]
pub struct Model3D {
    model: Vec<Vec<Vec<i32>>>,
    model_mask: Vec<Vec<Vec<u8>>>,
    borders: Vec<Vec<Vec<i32>>>,
    layers_filling_values: Vec<Vec<i32>>,
    max_depth: i32,
    params: Params3D,
}

impl Model3D {
    pub fn new(
        model: Vec<Vec<Vec<i32>>>,
        model_mask: Vec<Vec<Vec<u8>>>,
        borders: Vec<Vec<Vec<i32>>>,
        layers_filling_values: Vec<Vec<i32>>,
        max_depth: i32,
        params: Params3D) -> Model3D {
        Model3D {
            model,
            model_mask,
            borders,
            layers_filling_values,
            max_depth,
            params,
        }
    } 
}

impl Model3D {
    pub fn model(&self) -> &Vec<Vec<Vec<i32>>> {
        &self.model
    }

    pub fn model_mask(&self) -> &Vec<Vec<Vec<u8>>> {
        &self.model_mask
    }
    
    pub fn borders(&self) -> &Vec<Vec<Vec<i32>>> {
        &self.borders
    }

    pub fn layers_filling_values(&self) -> &Vec<Vec<i32>> {
        &self.layers_filling_values
    }

    pub fn max_depth(&self) -> i32 {
        self.max_depth
    }

    pub fn params(&self) -> &Params3D {
        &self.params
    }
}

fn get_max_depth(borders: &[Vec<Vec<i32>>]) -> i32 {
    let mut max_elem = 0;
    // borders stores as Z->Y->X
    for depth in borders {
        for y_cord in depth {
            for x_cord_value in y_cord {
                if *x_cord_value > max_elem {
                    max_elem = *x_cord_value;
                } 
            }
        } 
    }
    max_elem
}
