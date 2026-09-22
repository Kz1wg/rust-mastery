//! Lesson 05-4: GATの基本用途
//!
//! Container::iter は &self を借用するイテレータを返す。そのイテレータの
//! lifetimeは呼び出しごとに変わるため、associated type自体がlifetime
//! パラメータを持つ必要がある（GAT）。

pub trait Container {
    type Iter<'a>: Iterator<Item = &'a i32>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

pub struct Numbers(pub Vec<i32>);

impl Container for Numbers {
    type Iter<'a> = std::slice::Iter<'a, i32>;

    fn iter(&self) -> Self::Iter<'_> {
        todo!("self.0.iter() を返してください")
    }
}
