//! Lesson 13-3: 失敗ケースのテスト
//!
//! テストは「失敗したこと」ではなく「どう失敗したか」まで確かめている。

#[derive(Debug, PartialEq)]
pub enum AgeError {
    Empty,
    NotANumber,
    TooOld,
}

/// 年齢を解析する。
/// - 前後の空白は無視する
/// - 空なら Empty
/// - u8 として解釈できなければ NotANumber（"300" のように u8 に入らない数も含む）
/// - 150 を超えたら TooOld
pub fn parse_age(s: &str) -> Result<u8, AgeError> {
    todo!("上の仕様どおりに Ok か、3種類の Err のどれかを返してください")
}

/// items の index 番目を返す。範囲外なら、呼び出し側のバグとして panic する。
/// panic のメッセージには "index out of range" を含めること。
pub fn get_item(items: &[i32], index: usize) -> i32 {
    // 注意: この todo! のメッセージに、テストが期待する文字列を書いてはいけない。
    // 書くと、未実装のまま should_panic(expected = ...) のテストが通ってしまう。
    todo!(
        "範囲外なら上の doc コメントの文字列を含めて panic し、そうでなければ要素を返してください"
    )
}
