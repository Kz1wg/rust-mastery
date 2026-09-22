pub fn even_squares(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .collect()
}

pub fn first_index_over(nums: &[i32], threshold: i32) -> Option<usize> {
    let mut sum = 0;
    for (i, &n) in nums.iter().enumerate() {
        sum += n;
        if sum > threshold {
            return Some(i);
        }
    }
    None
}
