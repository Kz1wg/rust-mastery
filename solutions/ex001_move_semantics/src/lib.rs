pub fn longest_owned(a: String, b: String) -> String {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}

pub fn longest_ref<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}
