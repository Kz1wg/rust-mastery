use ex004_compare_signatures::{dedup_sorted_iter, dedup_sorted_owned, dedup_sorted_ref};

fn sample() -> Vec<String> {
    vec!["banana", "apple", "banana", "cherry", "apple"]
        .into_iter()
        .map(String::from)
        .collect()
}

#[test]
fn owned_dedups_and_sorts() {
    let result = dedup_sorted_owned(sample());
    assert_eq!(result, vec!["apple", "banana", "cherry"]);
}

#[test]
fn ref_dedups_and_sorts_without_consuming_input() {
    let input = sample();
    let result = dedup_sorted_ref(&input);
    assert_eq!(result, vec!["apple", "banana", "cherry"]);
    // input はまだ使える（元のまま変更されていない）
    assert_eq!(input.len(), 5);
}

#[test]
fn iter_accepts_vec_of_string_via_reference() {
    let input = sample();
    let result = dedup_sorted_iter(&input);
    assert_eq!(result, vec!["apple", "banana", "cherry"]);
}

#[test]
fn iter_accepts_array_of_str_literals() {
    let result = dedup_sorted_iter(["banana", "apple", "banana"]);
    assert_eq!(result, vec!["apple", "banana"]);
}

#[test]
fn iter_accepts_vec_of_owned_strings() {
    let input: Vec<String> = vec!["b".to_string(), "a".to_string(), "b".to_string()];
    let result = dedup_sorted_iter(input);
    assert_eq!(result, vec!["a", "b"]);
}

#[test]
fn empty_input_is_handled() {
    assert_eq!(dedup_sorted_owned(vec![]), Vec::<String>::new());
    assert_eq!(dedup_sorted_ref(&[]), Vec::<String>::new());
    assert_eq!(
        dedup_sorted_iter(Vec::<String>::new()),
        Vec::<String>::new()
    );
}
