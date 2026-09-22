use ex014_generic_vs_dyn::{total_area_dyn, total_area_generic, Circle, Shape, Square};

#[test]
fn circle_area_is_correct() {
    let c = Circle { radius: 1.0 };
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
}

#[test]
fn square_area_is_correct() {
    let s = Square { side: 3.0 };
    assert_eq!(s.area(), 9.0);
}

#[test]
fn total_area_generic_sums_same_type() {
    let circles = vec![Circle { radius: 1.0 }, Circle { radius: 2.0 }];
    let expected = std::f64::consts::PI * (1.0 + 4.0);
    assert!((total_area_generic(&circles) - expected).abs() < 1e-9);
}

#[test]
fn total_area_dyn_sums_mixed_types() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    let expected = std::f64::consts::PI + 4.0;
    assert!((total_area_dyn(&shapes) - expected).abs() < 1e-9);
}

#[test]
fn total_area_dyn_with_empty_slice_is_zero() {
    let shapes: Vec<Box<dyn Shape>> = vec![];
    assert_eq!(total_area_dyn(&shapes), 0.0);
}
