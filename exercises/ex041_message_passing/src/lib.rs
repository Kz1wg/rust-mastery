//! Lesson 10-4: メッセージパッシング
//!
//! 結果を集めるだけなら、共有（Arc<Mutex<T>>）は要らない。
//! join の戻り値か、チャネルで受け取れば済む。

use std::sync::mpsc;
use std::thread;

/// データを2つに分け、各スレッドの戻り値（join）で合計する。
pub fn sum_with_join(data: Vec<u64>) -> u64 {
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let (left, right) = (left.to_vec(), right.to_vec());
    todo!("2つのスレッドを起動し、それぞれの合計を join で受け取って足してください")
}

/// データを workers 個に分け、各ワーカーの部分和をチャネルで集めて合計する。
///
/// 注意: 元の送信側（tx）を drop しないと、受信側が待ち続けて終わらない。
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
            let partial: u64 = todo!("chunk の合計を計算してください");
            tx.send(partial).unwrap();
        });
    }
    drop(tx);

    rx.iter().sum()
}

/// 受信側を drop した後に送信すると Err になることを確かめる。
pub fn send_to_closed_channel_fails() -> bool {
    let (tx, rx) = mpsc::channel::<u8>();
    drop(rx);
    let result = tx.send(1);
    todo!("result が Err かどうかを返してください")
}
