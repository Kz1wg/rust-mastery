use std::marker::PhantomData;

pub struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: u64) -> Self {
        Id {
            value,
            _marker: PhantomData,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

pub struct User {
    pub id: u64,
    pub name: String,
}

pub struct Order;

/// ```compile_fail
/// use ex034_phantom_data::{find_user, Id, Order};
///
/// let order_id: Id<Order> = Id::new(1);
/// find_user(&[], order_id); // Id<Order> は Id<User> ではない
/// ```
pub fn find_user(users: &[User], id: Id<User>) -> Option<&User> {
    users.iter().find(|u| u.id == id.value())
}
