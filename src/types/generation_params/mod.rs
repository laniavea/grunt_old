use serde::{Deserialize, Serialize};

use crate::types::{Axis, LayersDist, LayersBorder, LayersFill};
use crate::types::shifts::Shift3D;

mod params3d;
mod params2d;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Params3D {
    // Axes parameters 
    x_ax: Axis,
    y_ax: Axis,
    // Base layers parameters
    layers_dist: LayersDist,
    // How to modify layers
    layers_border: LayersBorder,
    // How to fill layers
    layers_fill: LayersFill,
    shifts: Vec<Shift3D>,
    // Optional params to reduce generation time
    model_needed: bool,
    mask_needed: bool,
    // Depth scaling
    depth_scale: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Params2D {
    x_ax: Axis,
}
