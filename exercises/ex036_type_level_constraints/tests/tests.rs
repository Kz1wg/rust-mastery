use ex036_type_level_constraints::{format_value, Meters, Seconds, Vector};

#[test]
fn dot_of_2d_vectors() {
    let a = Vector([1.0, 2.0]);
    let b = Vector([3.0, 4.0]);
    assert_eq!(a.dot(&b), 11.0);
}

#[test]
fn dot_of_3d_vectors() {
    let a = Vector([1.0, 0.0, 2.0]);
    let b = Vector([4.0, 5.0, 6.0]);
    assert_eq!(a.dot(&b), 16.0);
}

#[test]
fn add_is_componentwise() {
    let a = Vector([1.0, 2.0, 3.0]);
    let b = Vector([10.0, 20.0, 30.0]);
    assert_eq!(a.add(&b), Vector([11.0, 22.0, 33.0]));
}

#[test]
fn format_value_with_meters() {
    assert_eq!(format_value::<Meters>(3.0), "3m");
}

#[test]
fn format_value_with_seconds() {
    assert_eq!(format_value::<Seconds>(1.5), "1.5s");
}
