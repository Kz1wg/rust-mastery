use ex041_message_passing::{send_to_closed_channel_fails, sum_with_channel, sum_with_join};

#[test]
fn sum_with_join_adds_all() {
    assert_eq!(sum_with_join(vec![1, 2, 3, 4]), 10);
}

#[test]
fn sum_with_join_handles_odd_length_and_empty() {
    assert_eq!(sum_with_join(vec![1, 2, 3, 4, 5]), 15);
    assert_eq!(sum_with_join(vec![]), 0);
}

#[test]
fn sum_with_channel_matches_sequential_sum() {
    let data: Vec<u64> = (1..=100).collect();
    let expected: u64 = data.iter().sum();
    assert_eq!(sum_with_channel(data, 4), expected);
}

#[test]
fn sum_with_channel_with_more_workers_than_items() {
    assert_eq!(sum_with_channel(vec![1, 2, 3], 10), 6);
}

#[test]
fn sum_with_channel_with_empty_input() {
    assert_eq!(sum_with_channel(vec![], 4), 0);
}

#[test]
fn sending_to_a_closed_channel_is_an_error() {
    assert!(send_to_closed_channel_fails());
}
