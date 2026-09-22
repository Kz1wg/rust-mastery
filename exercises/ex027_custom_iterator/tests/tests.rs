use ex027_custom_iterator::Fibonacci;

#[test]
fn first_few_fibonacci_numbers() {
    let first_seven: Vec<u64> = Fibonacci::new().take(7).collect();
    assert_eq!(first_seven, vec![0, 1, 1, 2, 3, 5, 8]);
}

#[test]
fn take_zero_returns_empty() {
    let v: Vec<u64> = Fibonacci::new().take(0).collect();
    assert_eq!(v, Vec::<u64>::new());
}

#[test]
fn works_with_other_adapters() {
    // Iteratorを実装しただけで map / filter / sum が使えることを確認する
    let even_sum: u64 = Fibonacci::new().take(10).filter(|n| n % 2 == 0).sum();
    // 0,1,1,2,3,5,8,13,21,34 のうち偶数: 0,2,8,34
    assert_eq!(even_sum, 0 + 2 + 8 + 34);
}

#[test]
fn is_effectively_infinite() {
    // take(50) が有限時間で終わることを確認する（無限イテレータであることの間接確認）。
    // fib(94) は u64 を超え、この実装は2つ先を計算するので93個目の取得で溢れる。
    // そのため、それより十分小さい件数にする（Lesson 07-2 の注意を参照）。
    let count = Fibonacci::new().take(50).count();
    assert_eq!(count, 50);
}
