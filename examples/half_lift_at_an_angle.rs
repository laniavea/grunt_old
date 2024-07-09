use grunt::model3d::generate_model;
use grunt::types::*;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let mut params = generation_params::Params3D::new();

    params.set_x_axis(Axis::generate_axis(0.0, 90.0, None).unwrap());
    params.set_y_axis(Axis::generate_axis(0.0, 90.0, None).unwrap());

    params.set_layers_dist(LayersDist::create_from_vec([20, 30, 20].to_vec()).unwrap_or_default());

    let mut borders = LayersBorder::new();
    borders.set_border_deviation(5.0).unwrap();
    borders.set_border_max_step(Some(1));
    params.set_layers_border(borders);

    let mut fill = LayersFill::new();
    fill.set_is_preset_odreder(true);
    params.set_layers_fill(fill);

    let mut shift = shifts::Shift3D::new();
    shift.set_pos_y(-100.0);
    shift.set_angle_y(90.0).unwrap();
    shift.set_pos_x(45.1);
    shift.set_angle_x(80.0).unwrap();
    shift.set_angle_z(80.0).unwrap();
    shift.set_shift_force(10).unwrap();
    shift.set_shift_type(shifts::ShiftTypes::InnerDescent);
    shift.set_main_region(4).unwrap();
    params.add_shift(shift);

    let model = generate_model(params).unwrap();

    let save_state = vec!["params", "borders", "model", "model_mask"];
    let axis_export = vec![AxisExportType::AsNum, AxisExportType::AsNum, AxisExportType::AsNum];
    model.export_model("my_model", &save_state, &axis_export).unwrap();

    println!("generation succesfull, check my_model.json")
}
