use ex051_kinds_of_tests::word_frequency;

#[test]
fn counts_repeated_words_case_insensitively() {
    let freq = word_frequency("The cat and THE dog");
    assert_eq!(freq.get("the"), Some(&2));
    assert_eq!(freq.get("cat"), Some(&1));
}

#[test]
fn ignores_punctuation_around_words() {
    let freq = word_frequency("Yes! yes, YES.");
    assert_eq!(freq.get("yes"), Some(&3));
    assert_eq!(freq.len(), 1);
}

#[test]
fn empty_text_has_no_words() {
    assert!(word_frequency("").is_empty());
}

#[test]
fn words_made_only_of_punctuation_are_skipped() {
    let freq = word_frequency("hello ... !!! world");
    assert_eq!(freq.len(), 2);
}
