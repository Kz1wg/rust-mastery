pub fn first_then_push(v: &mut Vec<i32>, extra: i32) -> i32 {
    let first = v[0]; // i32 は Copy なので、値をコピーして取り出す
    v.push(extra);
    first
}

pub fn bump_two(v: &mut [i32], i: usize, j: usize) {
    if i == j {
        v[i] += 2;
        return;
    }
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    let (left, right) = v.split_at_mut(hi);
    left[lo] += 1;
    right[0] += 1;
}
