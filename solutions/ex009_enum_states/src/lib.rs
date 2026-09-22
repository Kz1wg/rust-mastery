#[derive(Debug, Clone, PartialEq)]
pub enum Connection {
    Disconnected { retry_count: u32 },
    Connected { retry_count: u32 },
    Failed { retry_count: u32, error: String },
}

impl Connection {
    pub fn new() -> Self {
        Connection::Disconnected { retry_count: 0 }
    }

    pub fn connect(self) -> Self {
        Connection::Connected {
            retry_count: self.retry_count(),
        }
    }

    pub fn fail(self, error: String) -> Self {
        Connection::Failed {
            retry_count: self.retry_count() + 1,
            error,
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self, Connection::Connected { .. })
    }

    pub fn retry_count(&self) -> u32 {
        match self {
            Connection::Disconnected { retry_count }
            | Connection::Connected { retry_count }
            | Connection::Failed { retry_count, .. } => *retry_count,
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}
