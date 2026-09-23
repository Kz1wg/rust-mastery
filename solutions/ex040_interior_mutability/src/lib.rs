use std::cell::{Cell, RefCell};

pub struct Counter {
    count: Cell<u32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count: Cell::new(0),
        }
    }

    pub fn increment(&self) {
        self.count.set(self.count.get() + 1);
    }

    pub fn get(&self) -> u32 {
        self.count.get()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Logger {
    entries: RefCell<Vec<String>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            entries: RefCell::new(Vec::new()),
        }
    }

    pub fn log(&self, message: &str) {
        self.entries.borrow_mut().push(message.to_string());
    }

    pub fn count(&self) -> usize {
        self.entries.borrow().len()
    }

    pub fn entries(&self) -> Vec<String> {
        self.entries.borrow().clone()
    }

    pub fn try_log(&self, message: &str) -> bool {
        match self.entries.try_borrow_mut() {
            Ok(mut entries) => {
                entries.push(message.to_string());
                true
            }
            Err(_) => false,
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
