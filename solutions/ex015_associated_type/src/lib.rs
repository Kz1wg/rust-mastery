pub trait Stack {
    type Item;

    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

pub struct IntStack {
    items: Vec<i32>,
}

impl IntStack {
    pub fn new() -> Self {
        IntStack { items: Vec::new() }
    }
}

impl Default for IntStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack for IntStack {
    type Item = i32;

    fn push(&mut self, item: i32) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
}

pub fn drain_all<S: Stack>(s: &mut S) -> Vec<S::Item> {
    let mut out = Vec::new();
    while let Some(x) = s.pop() {
        out.push(x);
    }
    out
}
