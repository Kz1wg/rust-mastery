//! Lesson 03-2: 状態遷移をenumで設計する
//!
//! ```text
//!             open                  lock(code)
//!  Open ◄───────────── Closed ─────────────────► Locked
//!    │                    ▲                          │
//!    └──────close─────────┘◄──────unlock(code)────────┘
//! ```
//!
//! 無効な遷移は `Err` を返し、状態を変更しない（Lesson の (A) 案）。

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

    /// Closed のときだけ成功する。
    pub fn open(&mut self) -> Result<(), DoorError> {
        todo!("Closed のときだけ Open にし、それ以外は Err を返してください")
    }

    /// Open のときだけ成功する。
    pub fn close(&mut self) -> Result<(), DoorError> {
        todo!("Open のときだけ Closed にし、それ以外は Err を返してください")
    }

    /// Closed のときだけ成功する。コードを保存する。
    pub fn lock(&mut self, code: u32) -> Result<(), DoorError> {
        todo!("Closed のときだけ Locked にし（code を保存）、それ以外は Err を返してください")
    }

    /// Locked かつコードが一致するときだけ成功する。
    /// コードが違う場合も Err（状態は変えない）。
    pub fn unlock(&mut self, code: u32) -> Result<(), DoorError> {
        todo!("Locked かつ code が一致する場合だけ Closed にし、それ以外は Err を返してください")
    }
}

impl Default for Door {
    fn default() -> Self {
        Self::new()
    }
}
