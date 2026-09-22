use ex019_trait_bounds::{describe, values_equal};

#[test]
fn describe_formats_integer() {
    assert_eq!(describe(42), "value: 42");
}

#[test]
fn describe_formats_string() {
    assert_eq!(describe("hi".to_string()), "value: hi");
}

#[test]
fn describe_works_with_a_type_that_has_no_default() {
    // std::fmt::Display さえ満たせば、Default が無い型でも呼べることを確認する
    struct NoDefault(i32);
    impl std::fmt::Display for NoDefault {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    assert_eq!(describe(NoDefault(7)), "value: 7");
}

#[test]
fn values_equal_true_case() {
    assert!(values_equal(3, 3));
}

#[test]
fn values_equal_false_case() {
    assert!(!values_equal(3, 4));
}

#[test]
fn values_equal_with_strings() {
    assert!(values_equal("a".to_string(), "a".to_string()));
}
