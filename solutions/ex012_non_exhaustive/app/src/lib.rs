use ex012_lib::Event;

pub fn handle(event: Event) -> String {
    match event {
        Event::Click { x, y } => format!("clicked at ({x}, {y})"),
        Event::Key(c) => format!("key pressed: {c}"),
        _ => "unknown event".to_string(),
    }
}
