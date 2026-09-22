pub fn pick_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}

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
    v.push(word);
}

pub fn collect_short_words(text: &str, max_len: usize) -> Vec<String> {
    text.split_whitespace()
        .filter(|w| w.len() <= max_len)
        .map(String::from)
        .collect()
}
