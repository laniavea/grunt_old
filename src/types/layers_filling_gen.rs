use crate::types::LayersFill;

impl Default for LayersFill {
    fn default() -> Self {
        Self::new()
    }
}

impl LayersFill {
    pub fn new() -> LayersFill {
        LayersFill {
            values_preset: vec![vec![100], vec![200], vec![300, 330]],
            is_preset_ordered: true,
            values_deviation: None,
            values_smooth: None,
            values_offset: None,
        }
    }
}

impl LayersFill {
    pub fn set_values_preset(&mut self, values: Vec<Vec<i32>>) -> Result<(), &'static str> {
        if values.is_empty() { return Err("Vector must contain at least one element")}
        for value in &values {
            if value.len() > 2 { return Err("Every sub vector can contain only 1 or 2 elements")}
        }
        self.values_preset = values;
        Ok(())
    }

    pub fn values_preset(&self) -> &Vec<Vec<i32>> {
        &self.values_preset
    }

    pub fn set_is_preset_odreder(&mut self, state: bool) {
        self.is_preset_ordered = state;
    }

    pub fn is_preset_ordered(&self) -> bool {
        self.is_preset_ordered
    }

    pub fn set_values_deviation(&mut self, deviation: Option<f32>) -> Result<(), &'static str> {
        if deviation.unwrap_or(1.0) <= 0.0 { return Err("deviation must be positive") };
        self.values_deviation = deviation;
        Ok(())
    }

    pub fn values_deviation(&self) -> Option<f32> {
        self.values_deviation
    }

    pub fn set_values_smooth(&mut self, smooth: Option<u32>) {
        self.values_smooth = smooth;
    }

    pub fn values_smooth(&self) -> Option<u32> {
        self.values_smooth
    }
    
    pub fn set_values_offset(&mut self, offset: Option<u32>) {
        self.values_offset = offset;
    }

    pub fn values_offset(&self) -> Option<u32> {
        self.values_offset
    }
}

