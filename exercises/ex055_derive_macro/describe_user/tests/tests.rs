use describe_user::{Config, Empty, User};

#[test]
fn type_name_is_generated() {
    assert_eq!(User::type_name(), "User");
    assert_eq!(Config::type_name(), "Config");
}

#[test]
fn field_names_are_generated_in_order() {
    assert_eq!(User::field_names(), vec!["name", "age"]);
    assert_eq!(Config::field_names(), vec!["path", "retries", "verbose"]);
}

/// 型の中の :: （std::path::PathBuf）をフィールド名と取り違えない。
#[test]
fn paths_in_types_are_not_mistaken_for_fields() {
    assert_eq!(Config::field_names().len(), 3);
}

#[test]
fn struct_without_fields() {
    assert_eq!(Empty::type_name(), "Empty");
    assert!(Empty::field_names().is_empty());
}
