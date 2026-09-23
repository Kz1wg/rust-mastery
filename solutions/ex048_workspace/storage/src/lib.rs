use ex048_core::Task;

#[derive(Default)]
pub struct MemoryStore {
    tasks: Vec<Task>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn pending_count(&self) -> usize {
        ex048_core::count_pending(&self.tasks)
    }
}
