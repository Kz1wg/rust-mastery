use ex056_unsafe_basics::{first_via_ptr, read_value, swap_values};

#[test]
fn first_via_ptr_reads_the_first_element() {
    assert_eq!(first_via_ptr(&[7, 8, 9]), Some(7));
}

#[test]
fn first_via_ptr_on_empty_slice_is_none() {
    assert_eq!(first_via_ptr(&[]), None);
}

#[test]
fn read_value_reads_through_a_valid_pointer() {
    let x = 42;
    // SAFETY: &x から作ったポインタで、x はこの行の間生きている。
    let v = unsafe { read_value(&x) };
    assert_eq!(v, 42);
}

#[test]
fn swap_values_swaps() {
    let mut a = 1;
    let mut b = 2;
    swap_values(&mut a, &mut b);
    assert_eq!((a, b), (2, 1));
}
