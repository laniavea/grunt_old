use crate::types::shifts::{Shift3D, ShiftTypes};

impl Default for Shift3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Shift3D {
    pub fn new() -> Shift3D {
        Shift3D {
            pos_x: 5.0,
            pos_y: 5.0,
            angle_x: 90.0,
            angle_y: 90.0,
            main_region: 2, // can be 1, 2, 3 or 4, means part of surface that is splitted by two lines
            angle_z: 90.0,
            shift_force: 20,
            shift_type: ShiftTypes::InnerDescent,
        }
    }
}

impl Shift3D {
    pub fn set_pos_x(&mut self, pos_x: f32) {
        self.pos_x = pos_x
    }

    pub fn pos_x(&self) -> f32 {
        self.pos_x
    }

    pub fn set_pos_y(&mut self, pos_y: f32) {
        self.pos_y = pos_y
    }

    pub fn pos_y(&self) -> f32 {
        self.pos_y
    }

    pub fn set_angle_x(&mut self, angle_x: f32) -> Result<(), &'static str> {
        if !(0.0..=180.0).contains(&angle_x) {
            return Err("Angle should be between 0.0 and 180.0");
        }
        self.angle_x = angle_x;
        Ok(())
    }

    pub fn angle_x(&self) -> f32 {
        self.angle_x
    }

    pub fn set_angle_y(&mut self, angle_y: f32) -> Result<(), &'static str> {
        if !(0.0..=180.0).contains(&angle_y) {
            return Err("Angle should be between 0.0 and 180.0");
        }
        self.angle_y = angle_y;
        Ok(())
    }

    pub fn angle_y(&self) -> f32 {
        self.angle_y
    }

    pub fn set_main_region(&mut self, region: i32) -> Result<(), &'static str> {
        if region <= 0 || region > 4 {
            return Err("Region can be only between 1 and 4, where 1 is upper left part, and 4 - lower right")
        }
        self.main_region = region;
        Ok(())
    }

    pub fn main_region(&self) -> i32 {
        self.main_region
    }

    pub fn set_angle_z(&mut self, angle_z: f32) -> Result<(), &'static str> {
        if !(0.0..=90.0).contains(&angle_z) {
            return Err("Angle_z should be between 0 and 90.0");
        }
        self.angle_z = angle_z;
        Ok(())
    }

    pub fn angle_z(&self) -> f32 {
        self.angle_z
    }

    pub fn set_shift_force(&mut self, shift_force: i32) -> Result<(), &'static str> {
        if shift_force < 0 {
            return Err("Shift force cannot be negative")
        }
        self.shift_force = shift_force;
        Ok(())
    }

    pub fn shift_force(&self) -> i32 {
        self.shift_force
    }

    pub fn set_shift_type(&mut self, shift_type: ShiftTypes) {
        self.shift_type = shift_type
    }

    pub fn shift_type(&self) -> ShiftTypes {
        self.shift_type.clone()
    }
}
