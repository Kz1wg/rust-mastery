use ex025_library_vs_application::{load_config, run_app, ConfigError};

#[test]
fn load_config_not_found() {
    let err = load_config("/definitely/does/not/exist.toml").unwrap_err();
    assert!(matches!(err, ConfigError::NotFound(_)));
}

#[test]
fn load_config_not_found_display_message() {
    let err = load_config("/definitely/does/not/exist.toml").unwrap_err();
    assert_eq!(err.to_string(), "設定ファイルが見つかりません");
}

#[test]
fn config_error_implements_std_error() {
    fn assert_is_error<E: std::error::Error>() {}
    assert_is_error::<ConfigError>();
}

#[test]
fn run_app_propagates_as_boxed_error() {
    let result = run_app("/definitely/does/not/exist.toml");
    assert!(result.is_err());
    // Box<dyn Error> に変換されても、Display は元のメッセージのまま
    let msg = result.unwrap_err().to_string();
    assert_eq!(msg, "設定ファイルが見つかりません");
}

#[test]
fn run_app_uppercases_content_on_success() {
    // tempfileクレートを使わず、既存の確実に読めるファイルを使う代わりに、
    // 一時ファイルを手動で作る。
    let path = std::env::temp_dir().join("ex025_test_config.txt");
    std::fs::write(&path, "hello").unwrap();
    let result = run_app(path.to_str().unwrap());
    std::fs::remove_file(&path).ok();
    assert_eq!(result.unwrap(), "HELLO");
}
