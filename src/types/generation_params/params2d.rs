use crate::types::generation_params::Params2D;
use crate::types::Axis;

impl Default for Params2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Params2D {
    pub fn new() -> Params2D {
        Params2D {
            x_ax: Axis::new(),
        }
    }
}

impl Params2D {
    pub fn set_x_axis(&mut self, axis: Axis) {
        self.x_ax = axis;
    }
    
    pub fn x_axis(&self) -> &Axis {
        &self.x_ax
    }
}
