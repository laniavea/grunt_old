use crate::types::{Axis, LayersDist, LayersBorder, LayersFill};
use crate::types::shifts::Shift3D;
use crate::types::generation_params::Params3D;

impl Default for Params3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Params3D {
    pub fn new() -> Params3D {
        Params3D {
            x_ax: Axis::new(),
            y_ax: Axis::new(),
            layers_dist: LayersDist::new(),
            layers_border: LayersBorder::new(),
            layers_fill: LayersFill::new(),
            shifts: Vec::new(),
            model_needed: true,
            mask_needed: true,
            depth_scale: 1.0,
        }
    }
}

impl Params3D {
    pub fn set_x_axis(&mut self, axis: Axis) {
        self.x_ax = axis;
    }
    
    pub fn x_axis(&self) -> &Axis {
        &self.x_ax
    }

    pub fn set_y_axis(&mut self, axis: Axis) {
        self.y_ax = axis;
    }

    pub fn y_axis(&self) -> &Axis {
        &self.y_ax
    }

    pub fn set_layers_dist(&mut self, layers: LayersDist) {
        self.layers_dist = layers;
    }

    pub fn layers_dist(&self) -> &LayersDist {
        &self.layers_dist
    }

    pub fn set_layers_border(&mut self, layers_border: LayersBorder) {
        self.layers_border = layers_border
    }

    pub fn layers_border(&self) -> &LayersBorder {
        &self.layers_border
    }

    pub fn set_layers_fill(&mut self, layers_fill: LayersFill) {
        self.layers_fill = layers_fill
    }

    pub fn layers_fill(&self) -> &LayersFill {
        &self.layers_fill
    }

    pub fn add_shift(&mut self, shift: Shift3D) {
        self.shifts.push(shift)
    }

    pub fn shifts(&self) -> &Vec<Shift3D> {
        &self.shifts
    }

    pub fn set_model_needed(&mut self, is_full_model: bool) {
        self.model_needed = is_full_model;
    }

    pub fn model_needed(&self) -> bool {
        self.model_needed
    }

    pub fn set_mask_needed(&mut self, is_mask: bool) {
        self.mask_needed = is_mask;
    }

    pub fn mask_needed(&self) -> bool {
        self.mask_needed
    }

    pub fn set_depth_scale(&mut self, depth_scale: f32) {
        self.depth_scale = depth_scale
    }

    pub fn depth_scale(&self) -> f32 {
        self.depth_scale
    }
}
