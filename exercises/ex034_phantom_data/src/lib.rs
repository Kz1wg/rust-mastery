//! Lesson 09-1: PhantomData
//!
//! `Id<T>` は中身が u64 だけの型付きID。T は値として持たないので PhantomData を使う。
//! Clone / Copy は derive せず手で実装する（derive すると T にも Clone / Copy を要求してしまう）。

use std::marker::PhantomData;

pub struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: u64) -> Self {
        todo!("value と PhantomData から Id を作ってください")
    }

    pub fn value(&self) -> u64 {
        todo!("中の番号を返してください")
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        todo!("Id は Copy なので、*self を返せば十分です")
    }
}

impl<T> Copy for Id<T> {}

/// User 自体は Copy ではない（name が String のため）。
pub struct User {
    pub id: u64,
    pub name: String,
}

pub struct Order;

/// Id<User> を受け取り、該当するユーザーを探す。
///
/// 注文の ID を渡すとコンパイルできない:
///
/// ```compile_fail
/// use ex034_phantom_data::{find_user, Id, Order};
///
/// let order_id: Id<Order> = Id::new(1);
/// find_user(&[], order_id); // Id<Order> は Id<User> ではない
/// ```
pub fn find_user(users: &[User], id: Id<User>) -> Option<&User> {
    todo!("users の中から id.value() と一致する id を持つユーザーを探してください")
}
