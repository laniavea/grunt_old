use crate::types::LayersBorder;
use crate::types::BorderModFuncParams;

impl Default for LayersBorder {
    fn default() -> LayersBorder {
        LayersBorder::new()
    }
}

impl LayersBorder {
    pub fn new() -> LayersBorder {
        LayersBorder {
            border_deviation: 0.0,
            border_mod_func: None,
            border_type: String::from("random"),
            border_max_step: None,
            border_step_prob: Some(0.5),
            borders_same_pattern: false,
            deviation_override: None,
        }
    }
}

impl LayersBorder {
    pub fn set_border_deviation(&mut self, border_deviation: f32) -> Result<(), &'static str> {
        if border_deviation < 0.0 {
            return Err("Border deviation can't be negative.")
        }
        self.border_deviation = border_deviation;
        Ok(())
    }

    pub fn border_deviation(&self) -> f32 {
        self.border_deviation
    }

    pub fn set_border_mod_func(&mut self, mod_func: Option<BorderModFuncParams>) {
        self.border_mod_func = mod_func;
    }

    pub fn border_mod_func(&self) -> Option<BorderModFuncParams> {
        self.border_mod_func
    }

    pub fn set_border_type(&mut self, border_type: String) -> Result<(), &'static str> {
        match border_type.as_str() {
            "random" => (),
            _ => return Err("border_type can be next: random"),
        }
        Ok(())
    }

    pub fn border_type(&self) -> &String {
        &self.border_type
    }

    pub fn set_border_max_step(&mut self, max_step: Option<i32>) {
        self.border_max_step = max_step
    }

    pub fn border_max_step(&self) -> Option<i32> {
        self.border_max_step
    }

    pub fn set_border_step_prob(&mut self, prob: Option<f32>) -> Result<(), &'static str> {
        if prob.is_none() {
            self.border_step_prob = prob
        } else {
            let prob = (prob.unwrap() * 1000.0).round() / 1000.0;
            if 0.0 < prob || prob > 1.0 {
                return Err("Probability must be between 0.0 and 1.0");
                
            }
            self.border_step_prob = Some(prob)
        }
        Ok(())
    }

    pub fn border_step_prob(&self) -> Option<f32> {
        self.border_step_prob
    }

    pub fn set_layers_same_deviation(&mut self, is_pattern_same: bool) {
        self.borders_same_pattern = is_pattern_same
    } 

    pub fn layers_same_deviation(&self) -> bool {
        self.borders_same_pattern
    } 

    pub fn set_deviation_override(&mut self, deviation_override: Option<Vec<[i32; 2]>>) -> Result<(), &'static str> {
        self.deviation_override = match deviation_override {
            Some(override_vec) => {
                if override_vec.is_empty() { return Err("Devitaion override vec cannot be empty") }
                for i in &override_vec {
                    if i[1] < i[0] {
                        return Err("Second element in vector's array must be bigger or equal first one")
                    }
                }
                Some(override_vec)
            }
            None => None
        };
        Ok(())
    }

    pub fn deviation_override(&self) -> &Option<Vec<[i32; 2]>> {
        &self.deviation_override
    }
}
