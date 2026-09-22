#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Click { x: i32, y: i32 },
    Key(char),
    Scroll { delta: i32 },
}
