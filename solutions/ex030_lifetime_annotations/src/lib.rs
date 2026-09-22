pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}

pub fn first<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}
