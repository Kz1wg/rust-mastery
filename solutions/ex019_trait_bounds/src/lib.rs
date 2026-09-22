pub fn describe<T: std::fmt::Display>(x: T) -> String {
    format!("value: {x}")
}

pub fn values_equal<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}
