pub fn apply_to_local<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let s = String::from("hello world");
    f(&s)
}

pub fn count_matching<F: Fn(&str) -> bool>(items: &[String], pred: F) -> usize {
    items.iter().filter(|s| pred(s)).count()
}
