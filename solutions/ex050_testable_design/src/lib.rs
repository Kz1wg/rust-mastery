pub fn greeting_for_hour(hour: u8) -> &'static str {
    if hour < 12 {
        "おはようございます"
    } else if hour < 18 {
        "こんにちは"
    } else {
        "こんばんは"
    }
}

pub fn is_business_hours(hour: u8) -> bool {
    (9..18).contains(&hour)
}

pub fn pick_with<'a, F>(items: &'a [&'a str], choose: F) -> Option<&'a str>
where
    F: Fn(usize) -> usize,
{
    if items.is_empty() {
        return None;
    }
    items.get(choose(items.len())).copied()
}
