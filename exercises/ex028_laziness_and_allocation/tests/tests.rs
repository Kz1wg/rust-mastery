use ex028_laziness_and_allocation::doubled;

#[test]
fn doubled_collected_into_vec() {
    let nums = vec![1, 2, 3];
    let result: Vec<i32> = doubled(&nums).collect();
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn doubled_summed_without_collecting() {
    let nums = vec![1, 2, 3, 4, 5];
    let total: i32 = doubled(&nums).sum();
    assert_eq!(total, 30);
}

#[test]
fn doubled_on_empty_slice() {
    let nums: Vec<i32> = vec![];
    let result: Vec<i32> = doubled(&nums).collect();
    assert_eq!(result, Vec::<i32>::new());
}
