use ex012_app::handle;
use ex012_lib::Event;

#[test]
fn handles_click() {
    let msg = handle(Event::Click { x: 3, y: 4 });
    assert!(msg.contains("3") && msg.contains("4"));
}

#[test]
fn handles_key() {
    let msg = handle(Event::Key('a'));
    assert!(msg.contains('a'));
}

/// Scroll は「あとから追加されたバリアント」という想定。
/// handle() が Click/Key しか具体的に扱っていなくても、
/// ワイルドカードの腕のおかげでコンパイルも実行も壊れないことを確認する。
#[test]
fn unknown_variant_falls_back_gracefully() {
    let msg = handle(Event::Scroll { delta: 5 });
    assert!(!msg.is_empty());
    // Click や Key 向けの説明文と混同されていないことだけ確認する
    assert!(!msg.contains("3, 4"));
}
