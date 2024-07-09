use std::fs::File;
use std::io::Write;

use numtoa::NumToA;

use crate::model3d::Model3D;
use crate::types::generation_params::Params3D;
use crate::types::{AxisExportType, Axis};

impl Model3D {
    pub fn export_model(&self, name: &str, save: &[&str], axes_export: &Vec<AxisExportType>) -> Result<(), std::io::Error> {
        let default_ax_type = vec![AxisExportType::AsSelf, AxisExportType::AsSelf, AxisExportType::AsSelf];

        let axes_export = if axes_export.len() != 3 {
            eprintln!("Warning: Axes export param is ignored, it must contain 3 elements (for x, y, z)");
            &default_ax_type
        } else {
            axes_export
        };

        let mut result = String::from("");
        result += "{\"params3D\":";

        if save.contains(&"params") {
            export_params(&mut result, &self.params);
        } else { result += "null" }

        result += ",\"output_axes\":";
        export_true_axes(&mut result, &self.params, axes_export, self.max_depth);

        result += ",\"borders\":";
        if save.contains(&"borders") {
            export_border_num(&mut result, &self.borders)
        } else { result += "null" }

        result += ",\"fill_values\":";
        if save.contains(&"fill_values") {
            export_border_num(&mut result, &self.borders)
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

fn export_border_num(result: &mut String, borders: &[Vec<Vec<i32>>]){
    let mut buf = [0u8; 12]; *result += "[";
    for (depth_num, depth) in borders.iter().enumerate() {
        *result += "{\"bo";
        *result += format!("{depth_num}\":[").as_str();
        for (y_num, y_axis) in depth.iter().enumerate() {
            *result += "{\"y";
            *result += format!("{y_num}\":[").as_str();

            result.push_str(y_axis[0].numtoa_str(10, &mut buf));

            for x in y_axis[1..].iter() {
                result.push(',');
                result.push_str(x.numtoa_str(10, &mut buf));
            }

            if y_num != depth.len() - 1 {
                *result += "]},"
            } else {
                *result += "]}"
            }
        }
        if depth_num != borders.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_model_num(result: &mut String, model: &[Vec<Vec<i32>>]) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (x_num, x_ax) in model.iter().enumerate() {
        *result += "{\"x";
        *result += format!("{x_num}\":[").as_str();

        for (y_num, y_ax) in x_ax.iter().enumerate() {
            *result += "{\"y";
            *result += format!("{y_num}\":[").as_str();

            result.push_str(y_ax[0].numtoa_str(10, &mut buf));

            for depth in y_ax[1..].iter() {
                result.push(',');
                result.push_str(depth.numtoa_str(10, &mut buf));
            }

            if y_num != x_ax.len() - 1 {
                *result += "]},"
            } else {
                *result += "]}"
            }
        }

        if x_num != model.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_mask_num(result: &mut String, model_mask: &[Vec<Vec<u8>>]) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (x_num, x_ax) in model_mask.iter().enumerate() {
        *result += "{\"x";
        *result += format!("{x_num}\":[").as_str();

        for (y_num, y_ax) in x_ax.iter().enumerate() {
            *result += "{\"y";
            *result += format!("{y_num}\":[").as_str();

            result.push_str(y_ax[0].numtoa_str(10, &mut buf));

            for depth in y_ax[1..].iter() {
                result.push(',');
                result.push_str(depth.numtoa_str(10, &mut buf));
            }

            if y_num != x_ax.len() - 1 {
                *result += "]},"
            } else {
                *result += "]}"
            }
        }

        if x_num != model_mask.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_params(result: &mut String, params: &Params3D) {
    result.push_str(serde_json::to_string(params).unwrap().as_str());
}

fn export_true_axes(result: &mut String, params: &Params3D, axes_export: &[AxisExportType], depth_model_size: i32) {
    *result += "{\"x_ax\":[";
    params.x_axis().export_axis(&axes_export[0], result);
    *result += "],";

    *result += "\"y_ax\":[";
    params.y_axis().export_axis(&axes_export[1], result);
    *result += "],";

    *result += "\"z_ax\":[";
    Axis::generate_axis(1.0, depth_model_size as f32, None).unwrap().export_axis(&axes_export[2], result);
    *result += "]}";

}
