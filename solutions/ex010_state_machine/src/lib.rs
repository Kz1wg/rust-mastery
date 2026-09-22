#[derive(Debug, Clone, PartialEq)]
pub enum DoorState {
    Open,
    Closed,
    Locked { code: u32 },
}

#[derive(Debug, PartialEq)]
pub struct DoorError;

pub struct Door {
    state: DoorState,
}

impl Door {
    pub fn new() -> Self {
        Door {
            state: DoorState::Open,
        }
    }

    pub fn state(&self) -> &DoorState {
        &self.state
    }

    pub fn open(&mut self) -> Result<(), DoorError> {
        match self.state {
            DoorState::Closed => {
                self.state = DoorState::Open;
                Ok(())
            }
            _ => Err(DoorError),
        }
    }

    pub fn close(&mut self) -> Result<(), DoorError> {
        match self.state {
            DoorState::Open => {
                self.state = DoorState::Closed;
                Ok(())
            }
            _ => Err(DoorError),
        }
    }

    pub fn lock(&mut self, code: u32) -> Result<(), DoorError> {
        match self.state {
            DoorState::Closed => {
                self.state = DoorState::Locked { code };
                Ok(())
            }
            _ => Err(DoorError),
        }
    }

    pub fn unlock(&mut self, code: u32) -> Result<(), DoorError> {
        match self.state {
            DoorState::Locked { code: locked_code } if locked_code == code => {
                self.state = DoorState::Closed;
                Ok(())
            }
            _ => Err(DoorError),
        }
    }
}

impl Default for Door {
    fn default() -> Self {
        Self::new()
    }
}
