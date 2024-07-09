use crate::types::shifts::{Shift2D, ShiftTypes2D};

impl Default for Shift2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Shift2D {
    pub fn new() -> Shift2D {
        Shift2D {
            pos_x: 5.0,
            angle: 90.0,
            shift_force: 20,
            shift_type: ShiftTypes2D::RightDescent,
        }
    }
}

impl Shift2D {
    pub fn set_pos_x(&mut self, pos_x: f32) {
        self.pos_x = pos_x
    }

    pub fn pos_x(&self) -> f32 {
        self.pos_x
    }

    pub fn set_angle(&mut self, angle: f32) -> Result<(), &'static str> {
        if angle <= 0.0 || angle >= 90.0 {
            return Err("Angle must be between 0.0 and 90.0")
        }
        self.angle = angle;
        Ok(())
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn set_shift_force(&mut self, shift_force: i32) {
        self.shift_force = shift_force
    }

    pub fn shift_force(&self) -> i32 {
        self.shift_force
    }

    pub fn set_shift_type(&mut self, shift_type: ShiftTypes2D) {
        self.shift_type = shift_type
    }

    pub fn shift_type(&self) -> ShiftTypes2D {
        self.shift_type.clone()
    }
}
