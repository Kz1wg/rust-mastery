use std::sync::mpsc;
use std::thread;

pub fn sum_with_join(data: Vec<u64>) -> u64 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let (left, right) = (left.to_vec(), right.to_vec());

    let h1 = thread::spawn(move || left.iter().sum::<u64>());
    let h2 = thread::spawn(move || right.iter().sum::<u64>());

    h1.join().unwrap() + h2.join().unwrap()
}

pub fn sum_with_channel(data: Vec<u64>, workers: usize) -> u64 {
    if data.is_empty() {
        return 0;
    }
    let workers = workers.max(1);
    let chunk_size = data.len().div_ceil(workers);
    let (tx, rx) = mpsc::channel();

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let tx = tx.clone();
        thread::spawn(move || {
            let partial: u64 = chunk.iter().sum();
            tx.send(partial).unwrap();
        });
    }
    drop(tx);

    rx.iter().sum()
}

pub fn send_to_closed_channel_fails() -> bool {
    let (tx, rx) = mpsc::channel::<u8>();
    drop(rx);
    tx.send(1).is_err()
}
