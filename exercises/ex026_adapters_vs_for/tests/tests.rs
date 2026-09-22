use ex026_adapters_vs_for::{even_squares, first_index_over};

#[test]
fn even_squares_basic() {
    assert_eq!(even_squares(&[1, 2, 3, 4, 5, 6]), vec![4, 16, 36]);
}

#[test]
fn even_squares_no_evens() {
    assert_eq!(even_squares(&[1, 3, 5]), Vec::<i32>::new());
}

#[test]
fn even_squares_empty() {
    assert_eq!(even_squares(&[]), Vec::<i32>::new());
}

#[test]
fn first_index_over_basic() {
    assert_eq!(first_index_over(&[1, 2, 3, 10, 1], 5), Some(2));
}

#[test]
fn first_index_over_first_element() {
    assert_eq!(first_index_over(&[100], 5), Some(0));
}

#[test]
fn first_index_over_never_exceeds() {
    assert_eq!(first_index_over(&[1, 1, 1], 100), None);
}
