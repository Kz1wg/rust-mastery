/// ```compile_fail
/// use std::rc::Rc;
///
/// let data = Rc::new(vec![1u64, 2, 3]);
/// let d = Rc::clone(&data);
/// std::thread::spawn(move || d.iter().sum::<u64>()); // Rc は Send ではない
/// ```
pub fn spawn_sum(data: Vec<u64>) -> u64 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let (left, right) = (left.to_vec(), right.to_vec());

    let h1 = std::thread::spawn(move || left.iter().sum::<u64>());
    let h2 = std::thread::spawn(move || right.iter().sum::<u64>());

    h1.join().unwrap() + h2.join().unwrap()
}

pub fn assert_send<T: Send>() {}

pub fn assert_sync<T: Sync>() {}
