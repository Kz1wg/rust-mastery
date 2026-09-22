/// ```compile_fail
/// use ex032_static_bound::describe_later;
///
/// let local = String::from("x");
/// let f = describe_later(&local); // &local は 'static を満たさない
/// ```
pub fn describe_later<T: std::fmt::Display + 'static>(item: T) -> Box<dyn Fn() -> String> {
    Box::new(move || item.to_string())
}
