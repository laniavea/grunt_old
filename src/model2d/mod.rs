use crate::types::generation_params::Params2D;

pub mod export;

#[derive(Debug, Clone)]
pub struct Model2D {
    model: Vec<Vec<i32>>,
    model_mask: Vec<Vec<u8>>,
    borders: Vec<Vec<i32>>,
    params: Params2D,
}

impl Model2D {
    pub fn new(
        model: Vec<Vec<i32>>,
        model_mask: Vec<Vec<u8>>,
        borders: Vec<Vec<i32>>,
        params: Params2D) -> Model2D {
        Model2D {
            model,
            model_mask,
            borders,
            params,
        }
    } 
}

impl Model2D {
    pub fn model(&self) -> &Vec<Vec<i32>> {
        &self.model
    }

    pub fn model_mask(&self) -> &Vec<Vec<u8>> {
        &self.model_mask
    }
    
    pub fn borders(&self) -> &Vec<Vec<i32>> {
        &self.borders
    }
}
