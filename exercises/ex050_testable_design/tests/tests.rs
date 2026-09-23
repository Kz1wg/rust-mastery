use ex050_testable_design::{greeting_for_hour, is_business_hours, pick_with};

#[test]
fn greeting_in_the_morning() {
    assert_eq!(greeting_for_hour(8), "おはようございます");
}

#[test]
fn greeting_in_the_afternoon_and_evening() {
    assert_eq!(greeting_for_hour(15), "こんにちは");
    assert_eq!(greeting_for_hour(21), "こんばんは");
}

/// 境目の時刻を確かめる。時刻を引数でもらえるので、境目もいつでもテストできる。
#[test]
fn greeting_at_boundaries() {
    assert_eq!(greeting_for_hour(0), "おはようございます");
    assert_eq!(greeting_for_hour(11), "おはようございます");
    assert_eq!(greeting_for_hour(12), "こんにちは");
    assert_eq!(greeting_for_hour(17), "こんにちは");
    assert_eq!(greeting_for_hour(18), "こんばんは");
    assert_eq!(greeting_for_hour(23), "こんばんは");
}

#[test]
fn business_hours_boundaries() {
    assert!(!is_business_hours(8));
    assert!(is_business_hours(9));
    assert!(is_business_hours(17));
    assert!(!is_business_hours(18));
}

/// 乱数の代わりに「いつも同じ位置を返す関数」を渡せば、結果は毎回同じになる。
#[test]
fn pick_with_uses_the_given_choice() {
    let items = ["red", "green", "blue"];
    assert_eq!(pick_with(&items, |_| 0), Some("red"));
    assert_eq!(pick_with(&items, |_| 2), Some("blue"));
    assert_eq!(pick_with(&items, |len| len - 1), Some("blue"));
}

#[test]
fn pick_with_on_empty_returns_none() {
    let items: [&str; 0] = [];
    assert_eq!(pick_with(&items, |_| 0), None);
}
