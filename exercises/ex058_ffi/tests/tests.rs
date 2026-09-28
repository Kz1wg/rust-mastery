use ex058_ffi::{c_abs, c_strlen, from_c_str, rust_add};
use std::ffi::CString;

#[test]
fn c_abs_works() {
    assert_eq!(c_abs(-7), Some(7));
    assert_eq!(c_abs(3), Some(3));
    assert_eq!(c_abs(0), Some(0));
    assert_eq!(c_abs(i32::MAX), Some(i32::MAX));
}

/// i32::MIN を C の abs に渡すと未定義動作になる。safe な関数なので、C に渡す前に弾く。
#[test]
fn c_abs_rejects_the_value_that_is_undefined_in_c() {
    assert_eq!(c_abs(i32::MIN), None);
}

#[test]
fn c_strlen_counts_bytes() {
    assert_eq!(c_strlen("hello"), Ok(5));
    assert_eq!(c_strlen(""), Ok(0));
    // 「あ」は UTF-8 で3バイト。C の strlen は文字数ではなくバイト数を数える
    assert_eq!(c_strlen("あ"), Ok(3));
}

#[test]
fn c_strlen_rejects_interior_nul() {
    assert!(c_strlen("a\0b").is_err());
}

#[test]
fn rust_add_is_callable_from_rust_too() {
    let sum = rust_add(2, 3);
    // 骨組みは todo!() の代わりに i32::MIN を返している（extern "C" の中では panic できないため）
    assert_ne!(
        sum,
        i32::MIN,
        "not yet implemented: rust_add はまだ仮の値（i32::MIN）を返しています"
    );
    assert_eq!(sum, 5);
}

#[test]
fn from_c_str_round_trips() {
    let c = CString::new("hello ffi").unwrap();
    // SAFETY: c は \0 終端の有効な C 文字列で、この行の間生きている。
    let s = unsafe { from_c_str(c.as_ptr()) };
    assert_eq!(s, "hello ffi");
}
