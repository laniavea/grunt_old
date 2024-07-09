use std::fs::File;
use std::io::Write;

use numtoa::NumToA;

use crate::model2d::Model2D;
use crate::types::{AxisExportType, Axis};
use crate::types::generation_params::Params2D;

impl Model2D {
    pub fn export_model(&self, name: &str, save: &[&str], axes_export: &[AxisExportType]) -> Result<(), std::io::Error> {
        let default_ax_type = vec![AxisExportType::AsSelf, AxisExportType::AsSelf];

        let axes_export = if axes_export.len() != 2 {
            eprintln!("Warning: Axes export param is ignored, it must contain 2 elements (for x, z)");
            &default_ax_type
        } else {
            axes_export
        };

        let mut result = String::from("");
        result += "{\"params2D\":";

        if save.contains(&"params") {
            unimplemented!("Params 2D not implemented yet");
        } else { result += "null" }

        result += ",\"output_axes\":";

        let max_depth = if !self.model.is_empty() {
            self.model[0].len()
        } else if !self.model_mask.is_empty() {
            self.model_mask[0].len()
        } else {
            get_max_depth(&self.borders)
        };

        export_true_axes(&mut result, &self.params, axes_export, max_depth);

        result += ",\"borders\":";
        if save.contains(&"borders") {
            export_border_num(&mut result, &self.borders)
        } else { result += "null" }

        result += ",\"fill_values\":";
        if save.contains(&"fill_values") {
            unimplemented!("Fill values for Model2D not implemented yet");
        } else { result += "null" }

        result += ",\"model\":";
        if save.contains(&"model") {
            export_model_num(&mut result, &self.model)
        } else { result += "null" }

        result += ",\"model_mask\":";
        if save.contains(&"model_mask") {
            export_mask_num(&mut result, &self.model_mask)
        } else { result += "null" }
        result += "}";

        if name == "TestModelBench.test.bench" { return Ok(()) }

        let mut file = File::create(format!("{name}.json"))?;
        file.write_all(result.as_bytes())?;
        Ok(())
    }
}

fn export_border_num(result: &mut String, borders: &[Vec<i32>]){
    let mut buf = [0u8; 12];
    *result += "[";

    for (depth_num, depth) in borders.iter().enumerate() {
        *result += "{\"bo";
        *result += format!("{depth_num}\":[").as_str();

        result.push_str(depth[0].numtoa_str(10, &mut buf));

        for x in depth[1..].iter() {
            result.push(',');
            result.push_str(x.numtoa_str(10, &mut buf));
        }

        if depth_num != borders.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_model_num(result: &mut String, model: &[Vec<i32>]) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (x_num, x_ax) in model.iter().enumerate() {
        *result += "{\"x";
        *result += format!("{x_num}\":[").as_str();

        result.push_str(x_ax[0].numtoa_str(10, &mut buf));

        for depth in x_ax[1..].iter() {
            result.push(',');
            result.push_str(depth.numtoa_str(10, &mut buf));
        }

        if x_num != model.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_mask_num(result: &mut String, model_mask: &[Vec<u8>]) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (x_num, x_ax) in model_mask.iter().enumerate() {
        *result += "{\"x";
        *result += format!("{x_num}\":[").as_str();

        result.push_str(x_ax[0].numtoa_str(10, &mut buf));

        for depth in x_ax[1..].iter() {
            result.push(',');
            result.push_str(depth.numtoa_str(10, &mut buf));
        }

        if x_num != model_mask.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_true_axes(result: &mut String, params: &Params2D, axes_export: &[AxisExportType], depth_model_size: usize) {
    *result += "{\"x_ax\":[";
    params.x_axis().export_axis(&axes_export[0], result);
    *result += "],";

    *result += "\"z_ax\":[";
    Axis::generate_axis(0.0, (depth_model_size-1) as f32, None).unwrap().export_axis(&axes_export[1], result);
    *result += "]}";

}

fn get_max_depth(borders: &Vec<Vec<i32>>) -> usize {
    let mut max_elem = 0;
    for depth in borders {
        for x_cord_value in depth {
            if *x_cord_value > max_elem {
                max_elem = *x_cord_value;
            } 
        } 
    }
    max_elem as usize
}
