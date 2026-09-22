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
        self.0.iter()
    }
}
