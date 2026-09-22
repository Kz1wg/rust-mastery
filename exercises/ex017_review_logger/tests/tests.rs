use ex017_review_logger::App;

#[test]
fn run_returns_log_formatted_startup_message() {
    let app = App::new();
    assert_eq!(app.run(), "[LOG] starting up");
}

#[test]
fn default_also_works() {
    let app = App::default();
    assert_eq!(app.run(), "[LOG] starting up");
}
