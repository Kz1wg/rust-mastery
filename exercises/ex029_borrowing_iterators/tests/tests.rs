use ex029_borrowing_iterators::merged_over;

#[test]
fn merged_over_combines_and_filters() {
    let a = vec![1, 5, 10];
    let b = vec![2, 20, 3];
    let result: Vec<i32> = merged_over(&a, &b, 4).copied().collect();
    assert_eq!(result, vec![5, 10, 20]);
}

#[test]
fn merged_over_preserves_order_across_slices() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let result: Vec<i32> = merged_over(&a, &b, 0).copied().collect();
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn merged_over_with_nothing_passing_threshold() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let result: Vec<i32> = merged_over(&a, &b, 100).copied().collect();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn merged_over_with_empty_slices() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![];
    let result: Vec<i32> = merged_over(&a, &b, 0).copied().collect();
    assert_eq!(result, Vec::<i32>::new());
}
