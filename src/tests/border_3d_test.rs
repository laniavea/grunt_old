use rand::Rng;

use super::types::{Axis, LayersDist, LayersBorder};
use super::types::generation_params::Params3D;
use super::model3d::borders3d::create_layers_borders_3d;

#[test]
fn random_gen_layers_borders_tests(){
    let gen_count = 5000;
    let mut rng = rand::thread_rng();

    for _ in 0..gen_count {
        let mut params = Params3D::new();

        params.set_x_axis(Axis::generate_axis(0i16, rng.gen_range(1..25) as i16, None).unwrap());
        params.set_y_axis(Axis::generate_axis(0i16, rng.gen_range(1..25) as i16, None).unwrap());

        let layer_num: u8 = rng.gen_range(2..20);
        let layer_min: i32 = rng.gen_range(1..20);
        let layer_max: i32 = rng.gen_range(layer_min..layer_min+20);
        let layer_sum: Option<i32> = Some(rng.gen_range(layer_min*layer_num as i32..layer_max*layer_num as i32 + 1));

        params.set_layers_dist(LayersDist::generate_from_params(layer_num, layer_min, layer_max, layer_sum).
            expect("Impossible to generate, recheck test"));

        let mut borders = LayersBorder::new();
        borders.set_border_deviation(rng.gen_range(0..100) as f32 / 10.0).expect("Error");
        borders.set_border_max_step(Some(rng.gen_range(0..100)));
        params.set_layers_border(borders);

        let res = create_layers_borders_3d(&params);
        assert!(res.is_ok());
    }
}

#[test]
fn check_border_mod_function() {
    fn test_function(x_cord: usize, y_cord: usize, layers_num: usize, z_value: i32) -> i32 {
        (x_cord * y_cord * layers_num) as i32 * z_value
    }

    let mut params = Params3D::new();
    params.set_x_axis(Axis::generate_axis(0, 2i16, None).unwrap());
    params.set_y_axis(Axis::generate_axis(0, 2i16, None).unwrap());

    params.set_layers_dist(LayersDist::create_from_vec(vec![1, 1, 1]).unwrap());

    let mut borders = LayersBorder::new();
    borders.set_border_mod_func(Some(test_function));
    params.set_layers_border(borders);

    let res = create_layers_borders_3d(&params).unwrap();
    assert_eq!(res, vec![vec![vec![1, 1], vec![1, 1]], vec![vec![2, 2], vec![2, 0]], vec![vec![3, 3], vec![3, -3]]])
}
