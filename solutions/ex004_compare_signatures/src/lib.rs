pub fn dedup_sorted_owned(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v.dedup();
    v
}

pub fn dedup_sorted_ref(v: &[String]) -> Vec<String> {
    let mut owned: Vec<String> = v.to_vec();
    owned.sort();
    owned.dedup();
    owned
}

pub fn dedup_sorted_iter<I>(v: I) -> Vec<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut owned: Vec<String> = v.into_iter().map(|s| s.as_ref().to_string()).collect();
    owned.sort();
    owned.dedup();
    owned
}
