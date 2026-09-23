#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, title: &str) -> Self {
        Task {
            id,
            title: title.to_string(),
            done: false,
        }
    }

    pub fn complete(&mut self) {
        self.done = true;
    }
}

pub fn count_pending(tasks: &[Task]) -> usize {
    tasks.iter().filter(|t| !t.done).count()
}
