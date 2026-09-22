pub fn sum_even_squares_iter(v: &[u64]) -> u64 {
    v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()
}

pub fn sum_even_squares_loop(v: &[u64]) -> u64 {
    let mut total = 0;
    for &x in v {
        if x % 2 == 0 {
            total += x * x;
        }
    }
    total
}

#[repr(transparent)]
pub struct UserId(u64);

impl UserId {
    pub fn new(value: u64) -> Self {
        UserId(value)
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}
