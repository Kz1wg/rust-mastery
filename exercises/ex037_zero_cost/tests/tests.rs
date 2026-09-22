use ex037_zero_cost::{sum_even_squares_iter, sum_even_squares_loop, UserId};
use std::mem::size_of;

#[test]
fn iter_version_is_correct() {
    let v: Vec<u64> = (1..=10).collect();
    assert_eq!(sum_even_squares_iter(&v), 4 + 16 + 36 + 64 + 100);
}

#[test]
fn both_versions_agree() {
    let v: Vec<u64> = (0..1000).collect();
    assert_eq!(sum_even_squares_iter(&v), sum_even_squares_loop(&v));
}

#[test]
fn empty_input_is_zero() {
    assert_eq!(sum_even_squares_iter(&[]), 0);
    assert_eq!(sum_even_squares_loop(&[]), 0);
}

#[test]
fn user_id_round_trips() {
    assert_eq!(UserId::new(7).get(), 7);
}

/// newtype は追加のメモリを使わない（repr(transparent) でレイアウトが保証される）。
#[test]
fn newtype_has_no_size_overhead() {
    assert_eq!(size_of::<UserId>(), size_of::<u64>());
}

/// Option<&T> は null を None として使うので、参照と同じ大きさ（言語として保証）。
#[test]
fn option_of_reference_has_no_size_overhead() {
    assert_eq!(size_of::<Option<&UserId>>(), size_of::<&UserId>());
}

/// Option<u64> には「使われない値」が無いので、判別用の領域が増える。
#[test]
fn option_of_integer_does_have_overhead() {
    assert!(size_of::<Option<u64>>() > size_of::<u64>());
}
