use rand::Rng; 

use super::types::LayersDist;

#[test]
fn random_gen_layers_dist_tests() {
    let gen_count = 20000;
    let mut rng = rand::thread_rng();
    
    let layers_num: Vec<u8> = (0..gen_count).map(|_| rng.gen_range(2..20)).collect();
    let layers_min: Vec<i32> = (0..gen_count).map(|_| rng.gen_range(1..20)).collect();
    let layers_max: Vec<i32> = (0..gen_count).map(|i| rng.gen_range(layers_min[i]..40)).collect();
    let layers_sum: Vec<Option<i32>> = (0..gen_count)
        .map(|i| Some(rng.gen_range(layers_min[i]*layers_num[i] as i32..layers_max[i]*layers_num[i] as i32 + 1)))
        .collect();

    let mut answers: Vec<Result<Vec<i32>, &'static str>> = vec![];

    for i in 0..gen_count {
        answers.push(LayersDist::generate_layers_dist_vec(
            layers_num[i],
            layers_min[i],
            layers_max[i],
            layers_sum[i])
        );
    }

    let mut errors = 0;
    'a: for (i, e)  in answers.iter().enumerate() {
        if e.is_err() { 
            errors += 1;
            continue;
        }

        let (mut new_layers_num, mut new_layers_sum) = (0, 0);

        for j in e.clone().unwrap() {
            new_layers_num += 1;
            new_layers_sum += j;
            if !(j >= layers_min[i] && j <= layers_max[i]) {
                errors += 1;
                continue 'a;
            }
        }

        if new_layers_num != layers_num[i] || new_layers_sum != layers_sum[i].unwrap() {
            errors += 1;
            continue;
        }
    }
    assert_eq!(errors, 0);
}

#[test]
fn static_gen_layers_dist_tests() {
    let ld = LayersDist::generate_from_params(0, 10, 20, None);
    assert!(ld.is_err());

    let ld = LayersDist::generate_from_params(1, 10, 20, Some(30));
    assert!(ld.is_err());

    let ld = LayersDist::generate_from_params(1, 10, 10, Some(10));
    assert_eq!(ld.unwrap().get_layers_dist()[0], 10);

    let ld = LayersDist::generate_from_params(255, 1, 1000, Some(255000));
    assert_eq!(ld.unwrap().get_layers_dist().iter().sum::<i32>(), 255000);

    let ld = LayersDist::generate_from_params(255, 1, 1000, Some(255));
    assert_eq!(ld.unwrap().get_layers_dist().iter().sum::<i32>(), 255);

    let ld = LayersDist::generate_from_params(3, 2, 3, Some(9));
    assert_eq!(*ld.unwrap().get_layers_dist(), vec![3, 3, 3])
}

#[test]
fn from_vec_layers_dist() {
    let ld = LayersDist::create_from_vec(vec![]);
    assert!(ld.is_err());

    let ld = LayersDist::create_from_vec(vec![10, 0, 3]);
    assert!(ld.is_err());

    let ld = LayersDist::create_from_vec(vec![10, 1, 3]);
    assert_eq!(*ld.unwrap().get_layers_dist_summed(), vec![10, 11, 14]);

    let ld = LayersDist::create_from_vec(vec![i32::MAX-1, 1]);
    assert_eq!(*ld.unwrap().get_layers_dist_summed(), vec![i32::MAX-1, i32::MAX]);

    let ld = LayersDist::create_from_vec(vec![i32::MAX-1, 2]);
    assert!(ld.is_err());

    let ld = LayersDist::create_from_vec(vec![1, 1]);
    assert_eq!(*ld.unwrap().get_layers_dist(), vec![1, 1]);
} 
