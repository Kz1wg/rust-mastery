//! Lesson 09-2: variance
//!
//! - pick_longer: &'static str をローカルな参照と混ぜて渡せる（&T は共変）
//! - push_word: &mut Vec<&'a str> の 'a は縮められない（&mut T は T について不変）
//! - collect_short_words: 所有型（String）で集めれば、lifetime の問題自体が起きない

/// 長い方を返す（同じ長さなら a）。
pub fn pick_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!("a と b の長さを比べ、長い方を返してください（同じなら a）")
}

/// v に word を追加する。
///
/// v の型が Vec<&'static str> のとき、ローカルな参照は入れられない:
///
/// ```compile_fail
/// use ex035_variance::push_word;
///
/// let mut v: Vec<&'static str> = vec!["static"];
/// {
///     let local = String::from("local");
///     push_word(&mut v, &local); // 'a = 'static に固定され、local は足りない
/// }
/// println!("{v:?}");
/// ```
pub fn push_word<'a>(v: &mut Vec<&'a str>, word: &'a str) {
    todo!("v に word を追加してください")
}

/// 空白区切りの単語のうち、長さ（バイト数）が max_len 以下のものを集める。
/// 参照ではなく String で返すので、元の text より長く使える。
pub fn collect_short_words(text: &str, max_len: usize) -> Vec<String> {
    todo!("split_whitespace で分け、長さで絞り込み、String にして集めてください")
}
