use ex032_static_bound::describe_later;

#[test]
fn works_with_owned_string() {
    let f = describe_later(String::from("hello"));
    assert_eq!(f(), "hello");
}

#[test]
fn works_with_integer() {
    let f = describe_later(42);
    assert_eq!(f(), "42");
}

#[test]
fn works_with_static_str_literal() {
    // &'static str は 'static を満たす
    let f = describe_later("literal");
    assert_eq!(f(), "literal");
}

#[test]
fn closure_can_be_called_many_times() {
    let f = describe_later(String::from("again"));
    assert_eq!(f(), "again");
    assert_eq!(f(), "again");
}

/// String は 'static を満たすが、「永遠に生きる」わけではない。
/// 元の String は move されたので、クロージャを捨てれば一緒に破棄される。
#[test]
fn static_bound_does_not_mean_lives_forever() {
    let s = String::from("temporary");
    let f = describe_later(s);
    assert_eq!(f(), "temporary");
    drop(f); // ここで String も破棄される
}
