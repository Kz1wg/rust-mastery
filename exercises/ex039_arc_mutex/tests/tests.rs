use ex039_arc_mutex::{parallel_increment, push_and_count, total_length_times_three};

#[test]
fn parallel_increment_counts_every_step() {
    assert_eq!(parallel_increment(4, 1000), 4000);
}

#[test]
fn parallel_increment_with_single_thread() {
    assert_eq!(parallel_increment(1, 10), 10);
}

#[test]
fn parallel_increment_with_zero_threads() {
    assert_eq!(parallel_increment(0, 100), 0);
}

#[test]
fn total_length_is_summed_by_three_readers() {
    let words = vec!["ab".to_string(), "cde".to_string()];
    // 2 + 3 = 5 を3スレッドがそれぞれ数える
    assert_eq!(total_length_times_three(words), 15);
}

#[test]
fn total_length_with_empty_input() {
    assert_eq!(total_length_times_three(vec![]), 0);
}

#[test]
fn push_and_count_adds_one() {
    let words = vec!["a".to_string()];
    assert_eq!(push_and_count(words, "b"), 2);
}
