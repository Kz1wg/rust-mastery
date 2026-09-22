pub fn merged_over<'a>(
    a: &'a [i32],
    b: &'a [i32],
    threshold: i32,
) -> impl Iterator<Item = &'a i32> {
    a.iter().chain(b.iter()).filter(move |&&x| x > threshold)
}
