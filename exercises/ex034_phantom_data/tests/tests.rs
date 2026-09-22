use ex034_phantom_data::{find_user, Id, Order, User};

fn users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "alice".to_string(),
        },
        User {
            id: 2,
            name: "bob".to_string(),
        },
    ]
}

#[test]
fn id_holds_its_value() {
    let id: Id<User> = Id::new(42);
    assert_eq!(id.value(), 42);
}

#[test]
fn find_user_finds_existing() {
    let users = users();
    let found = find_user(&users, Id::new(2)).map(|u| u.name.as_str());
    assert_eq!(found, Some("bob"));
}

#[test]
fn find_user_returns_none_for_missing() {
    let users = users();
    assert!(find_user(&users, Id::new(99)).is_none());
}

/// User は Copy ではないが、Id<User> はコピーできる。
/// Clone / Copy を derive していたら、この関数はコンパイルできない（E0382）。
#[test]
fn id_is_copy_even_if_t_is_not() {
    let a: Id<User> = Id::new(1);
    let b = a;
    let c = a;
    assert_eq!(b.value(), c.value());
}

#[test]
fn different_kinds_of_ids_coexist() {
    let user_id: Id<User> = Id::new(1);
    let order_id: Id<Order> = Id::new(1);
    // 番号が同じでも、型が違うので取り違えられない
    assert_eq!(user_id.value(), order_id.value());
}
