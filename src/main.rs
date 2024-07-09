use types::{Axis, LayersBorder, LayersFill};
use types::shifts::Shift3D;

use crate::types::AxisExportType;

pub mod types;
pub mod model3d;
pub mod model2d;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    fn _test_function(x_cord: usize, y_cord: usize, _layers_num: usize, _z_value: i32) -> i32 {
        if x_cord <= 6 && (4..=6).contains(&y_cord) {
            return -6
        }
        0
    }

    let mut params = types::generation_params::Params3D::new();

    params.set_x_axis(Axis::generate_axis(0.0, 10.0, None).unwrap());
    params.set_y_axis(Axis::generate_axis(0.0, 10.0, None).unwrap());

    let mut borders = LayersBorder::new();
    borders.set_border_max_step(Some(3));
    borders.set_deviation_override(Some(vec![[5, 10], [11, 12], [20, 20]])).unwrap();
    // borders.set_border_mod_func(Some(_test_function));
    params.set_layers_border(borders);

    let mut fill = LayersFill::new();
    fill.set_values_preset(vec![vec![100], vec![200], vec![300, 1000]]).unwrap();
    fill.set_is_preset_odreder(true);
    params.set_layers_fill(fill);

    let mut shift = Shift3D::new();
    shift.set_pos_y(5.0);
    shift.set_angle_y(90.0).unwrap();
    shift.set_pos_x(12.0);
    shift.set_angle_x(90.0).unwrap();
    shift.set_angle_z(90.0).unwrap();
    shift.set_shift_force(10).unwrap();
    shift.set_shift_type(types::shifts::ShiftTypes::InnerDescent);
    shift.set_main_region(1).unwrap();
    params.add_shift(shift);

    println!("{:?}", params);
    let model = model3d::generate_model(params).unwrap();

    use std::time::Instant;
    let now = Instant::now();

    let save_state = ["params", "borders", "model", "model_mask"];
    let axis_export = vec![AxisExportType::AsSelf, AxisExportType::AsSelf, AxisExportType::AsSelf];
    model.export_model("my_model", &save_state, &axis_export).unwrap();

    let elapsed = now.elapsed();

    if !cfg!(test) {
        println!("Elapsed export: {:.2?}", elapsed);
    }

    let model_2d = model.to_model_2d_by_angle(5.0, 90.0, 15).unwrap();
    println!("2d Model depth is {}", model_2d.model()[0].len());
    let save_state = ["borders", "model", "model_mask"];
    let axis_export = vec![AxisExportType::AsSelf, AxisExportType::AsSelf];
    model_2d.export_model("my_model2D", &save_state, &axis_export).unwrap();
}

#[cfg(test)]
mod tests;
