use ex018_generic_benefit::min_max;

#[test]
fn min_max_on_integers() {
    let v = vec![34, 50, 25, 100, 65];
    assert_eq!(min_max(&v), Some((25, 100)));
}

#[test]
fn min_max_on_floats() {
    let v = vec![3.5, 1.2, 9.9, 4.4];
    assert_eq!(min_max(&v), Some((1.2, 9.9)));
}

#[test]
fn min_max_on_chars() {
    let v = vec!['y', 'm', 'a', 'q'];
    assert_eq!(min_max(&v), Some(('a', 'y')));
}

#[test]
fn min_max_on_single_element() {
    let v = vec![42];
    assert_eq!(min_max(&v), Some((42, 42)));
}

#[test]
fn min_max_on_empty_slice() {
    let v: Vec<i32> = vec![];
    assert_eq!(min_max(&v), None);
}
