use super::types::Axis;

#[test]
fn gen_axis_tests() {
    let ax = Axis::generate_axis(1.0, 3.0, None).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.0, 3.0]);
    assert_eq!(*ax.centers(), vec![1.5, 2.5]);

    let ax = Axis::generate_axis(1.0, 3.0, Some(1.0)).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.0, 3.0]);
    assert_eq!(*ax.centers(), vec![1.5, 2.5]);

    let ax = Axis::generate_axis(1.0, 3.0, Some(1.1)).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.1]);
    assert_eq!(*ax.centers(), vec![1.55]);

    let ax = Axis::generate_axis(1.0, 3.1, None).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.0, 3.0]);
    assert_eq!(*ax.centers(), vec![1.5, 2.5]);

    let ax = Axis::generate_axis(1.0, 3.1, Some(-1.0));
    assert!(ax.is_err());
        
    let ax = Axis::generate_axis(1.0, -3.0, Some(-1.0));
    assert!(ax.is_err());

    let ax = Axis::generate_axis(1.0, 3.1, Some(0.0));
    assert!(ax.is_err());

    let ax = Axis::generate_axis(1.0, 1.0, Some(1.0));
    assert!(ax.is_err());

    let ax = Axis::generate_axis(1.0, 1.9, Some(1.0));
    assert!(ax.is_err());

    let ax = Axis::generate_axis_hard(1.0, 3.0, None).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.0, 3.0]);
    assert_eq!(*ax.centers(), vec![1.5, 2.5]);

    let ax = Axis::generate_axis_hard(1.0, 3.0, Some(1.0)).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.0, 3.0]);
    assert_eq!(*ax.centers(), vec![1.5, 2.5]);

    let ax = Axis::generate_axis_hard(1.0, 3.0, Some(1.1)).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.1, 3.0]);
    assert_eq!(*ax.centers(), vec![1.55, 2.55]);

    let ax = Axis::generate_axis_hard(1.0, 3.1, None).unwrap();
    assert_eq!(*ax.axis(), vec![1.0, 2.0, 3.0, 3.1]);
    assert_eq!(*ax.centers(), vec![1.5, 2.5, 3.05]);

    println!("{:?}", *ax.axis());
    println!("{:?}", 1000.005f32 * 3.0);
    let ax = Axis::generate_axis_hard(6000.0, 9100.0, Some(1000.005)).unwrap();
    assert_eq!(*ax.axis(), vec![6000.0, 7000.005, 8000.01, 9000.015, 9100.0]);

    // TODO: Add more tests
}
