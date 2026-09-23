use ex057_safe_abstraction::{first_and_last_mut, my_split_at_mut};

#[test]
fn split_and_mutate_both_halves() {
    let mut v = [1, 2, 3, 4];
    let (a, b) = my_split_at_mut(&mut v, 1);
    a[0] = 10;
    b[0] = 20;
    assert_eq!(v, [10, 20, 3, 4]);
}

#[test]
fn split_at_the_ends() {
    let mut v = [1, 2, 3];
    {
        let (a, b) = my_split_at_mut(&mut v, 0);
        assert!(a.is_empty());
        assert_eq!(b.len(), 3);
    }
    let (a, b) = my_split_at_mut(&mut v, 3);
    assert_eq!(a.len(), 3);
    assert!(b.is_empty());
}

#[test]
fn split_works_for_other_types() {
    let mut words = vec!["a".to_string(), "b".to_string()];
    let (x, y) = my_split_at_mut(&mut words, 1);
    std::mem::swap(&mut x[0], &mut y[0]);
    assert_eq!(words, vec!["b", "a"]);
}

/// 範囲外の mid は、メモリを壊さず panic で止まる。
#[test]
#[should_panic(expected = "out of bounds")]
fn split_out_of_bounds_panics() {
    let mut v = [1, 2, 3];
    my_split_at_mut(&mut v, 4);
}

#[test]
fn first_and_last_mut_modifies_both_ends() {
    let mut v = [1, 2, 3, 4];
    if let Some((first, last)) = first_and_last_mut(&mut v) {
        *first = 100;
        *last = 400;
    }
    assert_eq!(v, [100, 2, 3, 400]);
}

#[test]
fn first_and_last_mut_needs_two_elements() {
    let mut one = [1];
    assert!(first_and_last_mut(&mut one).is_none());
    let mut empty: [i32; 0] = [];
    assert!(first_and_last_mut(&mut empty).is_none());
}
