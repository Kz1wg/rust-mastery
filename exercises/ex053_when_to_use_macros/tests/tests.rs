use ex053_when_to_use_macros::{max_of, square};

#[test]
fn square_basic() {
    assert_eq!(square(3), 9);
    assert_eq!(square(-4), 16);
}

fn next(counter: &mut i64) -> i64 {
    *counter += 1;
    *counter
}

/// 関数なので、next は1回しか呼ばれない（マクロだと2回呼ばれて結果が 1 * 2 = 2 になる）。
#[test]
fn square_evaluates_argument_once() {
    let mut c = 0;
    let r = square(next(&mut c));
    assert_eq!(r, 1);
    assert_eq!(c, 1);
}

#[test]
fn max_of_single_argument() {
    assert_eq!(max_of!(5), 5);
}

#[test]
fn max_of_many_arguments() {
    assert_eq!(max_of!(3, 9, 2, 7), 9);
    assert_eq!(max_of!(1, 2), 2);
    assert_eq!(max_of!(10, 2, 3), 10);
}

#[test]
fn max_of_works_with_other_types() {
    assert_eq!(max_of!(1.5, 0.5), 1.5);
    assert_eq!(max_of!("apple", "banana"), "banana");
}
