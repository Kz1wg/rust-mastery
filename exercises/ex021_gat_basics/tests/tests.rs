use ex021_gat_basics::{Container, Numbers};

#[test]
fn iter_yields_all_elements() {
    let n = Numbers(vec![1, 2, 3]);
    let collected: Vec<i32> = n.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn iter_sum_works() {
    let n = Numbers(vec![10, 20, 30]);
    let sum: i32 = n.iter().sum();
    assert_eq!(sum, 60);
}

#[test]
fn iter_on_empty_numbers() {
    let n = Numbers(vec![]);
    let collected: Vec<i32> = n.iter().copied().collect();
    assert_eq!(collected, Vec::<i32>::new());
}

/// iter() を複数回呼んでも、それぞれ独立した（&selfを借用する）
/// イテレータが返ることを確認する。
#[test]
fn iter_can_be_called_multiple_times() {
    let n = Numbers(vec![1, 2, 3]);
    let first_sum: i32 = n.iter().sum();
    let second_sum: i32 = n.iter().sum();
    assert_eq!(first_sum, second_sum);
}
