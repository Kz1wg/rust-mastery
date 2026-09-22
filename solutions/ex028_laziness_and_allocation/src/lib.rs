pub fn doubled(nums: &[i32]) -> impl Iterator<Item = i32> + '_ {
    nums.iter().map(|x| x * 2)
}
