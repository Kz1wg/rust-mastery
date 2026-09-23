use ex060_semver_friendly::{Config, Level, Store};

#[test]
fn config_new_uses_defaults() {
    let c = Config::new("/tmp/app");
    assert_eq!(c.path, "/tmp/app");
    assert_eq!(c.retries, 3);
    assert!(!c.verbose);
}

#[test]
fn config_with_methods_override_defaults() {
    let c = Config::new("/tmp/app").with_retries(5).with_verbose(true);
    assert_eq!(c.retries, 5);
    assert!(c.verbose);
}

#[test]
fn level_labels() {
    assert_eq!(Level::Low.label(), "low");
    assert_eq!(Level::High.label(), "high");
}

/// 利用者の match は、ワイルドカードを書いておけば、バリアントが増えても壊れない。
#[test]
fn user_side_match_with_wildcard() {
    fn is_urgent(level: Level) -> bool {
        match level {
            Level::High => true,
            _ => false, // 将来のバリアントにも対応できる
        }
    }
    assert!(is_urgent(Level::High));
    assert!(!is_urgent(Level::Low));
}

/// 利用者の実装。get だけを書けば、describe はデフォルト実装が使われる。
struct Fixed(u32);

impl Store for Fixed {
    fn get(&self) -> u32 {
        self.0
    }
}

#[test]
fn default_method_works_for_existing_implementations() {
    assert_eq!(Fixed(7).describe(), "value = 7");
}
