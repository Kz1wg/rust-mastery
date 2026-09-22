pub fn min_max<T: PartialOrd + Copy>(list: &[T]) -> Option<(T, T)> {
    if list.is_empty() {
        return None;
    }
    let mut min = list[0];
    let mut max = list[0];
    for &item in list {
        if item < min {
            min = item;
        }
        if item > max {
            max = item;
        }
    }
    Some((min, max))
}
