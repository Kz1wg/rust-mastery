//! Lesson 13-1: テスト可能な設計
//!
//! どの関数も、現在時刻や乱数を自分では読まない。必要なものは引数でもらう。
//! そのおかげで、テストはいつ実行しても同じ結果になる。

/// 時刻（時, 0〜23）に応じたあいさつを返す。
/// 12時より前は「おはようございます」、18時より前は「こんにちは」、それ以外は「こんばんは」。
pub fn greeting_for_hour(hour: u8) -> &'static str {
    todo!("hour の値で3つのあいさつを返し分けてください")
}

/// 営業時間（9時以上、18時未満）なら true。
pub fn is_business_hours(hour: u8) -> bool {
    todo!("9 以上かつ 18 未満なら true を返してください")
}

/// items の中から1つ選んで返す。どれを選ぶかは choose が決める。
///
/// choose は「要素数を受け取って、選ぶ位置を返す関数」。
/// 本番では乱数を使う関数を渡し、テストでは決まった位置を返す関数を渡す。
/// items が空なら None を返す（choose は呼ばない）。
pub fn pick_with<'a, F>(items: &'a [&'a str], choose: F) -> Option<&'a str>
where
    F: Fn(usize) -> usize,
{
    todo!("空なら None、そうでなければ choose(items.len()) の位置の要素を返してください")
}
