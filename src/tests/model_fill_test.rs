use rand::Rng;
use rand::distributions::Uniform;
use crate::model3d::fill3d::filling_model_3d::*; use crate::model3d::fill3d::GenerationTypes;

#[test]
fn fill_model_and_mask_tests() {
    let mut rnd = rand::thread_rng();
    let mut errors = 0;

    'a: for _ in 0..10000 {
        let model_size = rnd.gen_range(1..10);
        let model_x = rnd.gen_range(1..10);
        let model_y = rnd.gen_range(1..10);

        let mut borders: Vec<Vec<Vec<i32>>> = Vec::new();

        for i in 0..model_size {
            borders.push(Vec::new());
            for j in 0..model_y {
                borders[i].push(Vec::new());
                for k in 0..model_x {
                    let pr_value = if i != 0 {
                        borders[i-1][j][k]
                    } else {
                        1
                    };

                    borders[i][j].push(rnd.gen_range(pr_value..11));
                }
            }
        }

        let filling_values = vec![
            GenerationTypes::GenerationExact(1),
            GenerationTypes::GenerationExact(2),
            GenerationTypes::GenerationExact(3),
            GenerationTypes::GenerationExact(4),
            GenerationTypes::GenerationExact(5),
            GenerationTypes::GenerationExact(6),
            GenerationTypes::GenerationExact(7),
            GenerationTypes::GenerationExact(8),
            GenerationTypes::GenerationRange(Uniform::from(9..10)),
            GenerationTypes::GenerationRange(Uniform::from(11..255)),
        ];

        let (model, model_mask) = create_full_model_with_mask(&borders, &filling_values);

        let model_x = create_full_model_without_mask(&borders, &filling_values);
        let mask_x = create_only_mask(&borders);

        if model_x != model || mask_x != model_mask {
            errors += 1;
            break 'a
        }

        for i in 0..model.len() {
            for j in 0..model[0].len() {
                for k in 0..model[0][0].len() {
                    let model_value = model[i][j][k];

                    if model_value < 9 {
                        if (model_value - 1) as u8 != model_mask[i][j][k] { errors += 1 }
                    } else {
                        if (model_value == 9 || model_value == 10) && model_mask[i][j][k] != 8 { errors += 1 } 
                        if (model_value > 10 && model_value < 255) && model_mask[i][j][k] != 9 { errors += 1 }
                    }

                    if model_mask[i][j][k] as usize == model_size - 1 { continue; }

                    if borders[model_mask[i][j][k] as usize][j][i] < k as i32 {
                        errors += 1;
                        break 'a
                    }
                }
            }
        }
    }

    assert_eq!(errors, 0);
}
