use ex033_hrtb::{apply_to_local, count_matching};

#[test]
fn apply_to_local_passes_the_local_string() {
    assert_eq!(apply_to_local(|s| s.len()), "hello world".len());
}

#[test]
fn apply_to_local_accepts_function_items() {
    assert_eq!(apply_to_local(str::len), 11);
}

#[test]
fn apply_to_local_with_word_count() {
    assert_eq!(apply_to_local(|s| s.split_whitespace().count()), 2);
}

#[test]
fn count_matching_basic() {
    let items = vec![
        "apple".to_string(),
        "avocado".to_string(),
        "banana".to_string(),
    ];
    assert_eq!(count_matching(&items, |s| s.starts_with('a')), 2);
}

#[test]
fn count_matching_empty() {
    let items: Vec<String> = vec![];
    assert_eq!(count_matching(&items, |_| true), 0);
}
