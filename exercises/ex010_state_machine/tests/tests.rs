use ex010_state_machine::{Door, DoorError, DoorState};

#[test]
fn new_door_is_open() {
    let door = Door::new();
    assert_eq!(*door.state(), DoorState::Open);
}

#[test]
fn open_to_closed_to_open() {
    let mut door = Door::new();
    assert_eq!(door.close(), Ok(()));
    assert_eq!(*door.state(), DoorState::Closed);
    assert_eq!(door.open(), Ok(()));
    assert_eq!(*door.state(), DoorState::Open);
}

#[test]
fn cannot_close_an_already_closed_door() {
    let mut door = Door::new();
    door.close().unwrap();
    assert_eq!(door.close(), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Closed); // 変更されていない
}

#[test]
fn cannot_open_an_already_open_door() {
    let mut door = Door::new();
    assert_eq!(door.open(), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Open);
}

#[test]
fn cannot_lock_an_open_door() {
    let mut door = Door::new();
    assert_eq!(door.lock(1234), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Open);
}

#[test]
fn lock_a_closed_door_and_unlock_with_correct_code() {
    let mut door = Door::new();
    door.close().unwrap();
    assert_eq!(door.lock(1234), Ok(()));
    assert_eq!(*door.state(), DoorState::Locked { code: 1234 });

    assert_eq!(door.unlock(1234), Ok(()));
    assert_eq!(*door.state(), DoorState::Closed);
}

#[test]
fn unlock_with_wrong_code_fails_and_keeps_locked() {
    let mut door = Door::new();
    door.close().unwrap();
    door.lock(1234).unwrap();

    assert_eq!(door.unlock(9999), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Locked { code: 1234 });
}

#[test]
fn cannot_open_a_locked_door() {
    let mut door = Door::new();
    door.close().unwrap();
    door.lock(1234).unwrap();

    assert_eq!(door.open(), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Locked { code: 1234 });
}

#[test]
fn cannot_close_a_locked_door() {
    let mut door = Door::new();
    door.close().unwrap();
    door.lock(1234).unwrap();

    assert_eq!(door.close(), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Locked { code: 1234 });
}

#[test]
fn cannot_unlock_a_closed_door() {
    let mut door = Door::new();
    door.close().unwrap();

    assert_eq!(door.unlock(1234), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Closed);
}

#[test]
fn cannot_unlock_an_open_door() {
    let mut door = Door::new();
    assert_eq!(door.unlock(1234), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Open);
}

#[test]
fn cannot_lock_a_locked_door() {
    let mut door = Door::new();
    door.close().unwrap();
    door.lock(1234).unwrap();

    assert_eq!(door.lock(5678), Err(DoorError));
    assert_eq!(*door.state(), DoorState::Locked { code: 1234 }); // 変更されていない
}
