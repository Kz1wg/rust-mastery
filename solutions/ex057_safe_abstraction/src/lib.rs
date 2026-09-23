pub fn my_split_at_mut<T>(s: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = s.len();
    assert!(mid <= len, "mid ({mid}) is out of bounds for length {len}");
    let ptr = s.as_mut_ptr();
    // SAFETY:
    // - mid <= len を上で確認したので、2つの範囲はどちらもスライスの中に収まる
    // - [0, mid) と [mid, len) は重ならないので、同時に可変で持っても競合しない
    // - 戻り値の lifetime は s の借用と同じなので、元のスライスより長く生きない
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn first_and_last_mut<T>(s: &mut [T]) -> Option<(&mut T, &mut T)> {
    if s.len() < 2 {
        return None;
    }
    let (front, back) = my_split_at_mut(s, 1);
    Some((&mut front[0], back.last_mut()?))
}
