pub fn shout(x: impl std::fmt::Display) -> String {
    x.to_string().to_uppercase()
}

pub fn evens_up_to(n: i32) -> impl Iterator<Item = i32> {
    (0..n).filter(|x| x % 2 == 0)
}
