//! Lesson 10-2: 共有と可変性
//!
//! Arc は「複数スレッドが所有する」問題を、Mutex は「同時に変更されると困る」問題を解く。
//! 両方必要なので Arc<Mutex<T>> になる。

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

/// threads 個のスレッドがそれぞれ per_thread 回カウンタを加算し、最終的な合計を返す。
pub fn parallel_increment(threads: usize, per_thread: u64) -> u64 {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();

    for _ in 0..threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            todo!("per_thread 回だけ、ロックを取って中の値に 1 を足してください")
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    let total = *counter.lock().unwrap();
    total
}

/// words を3つのスレッドから読み、それぞれが数えた「全単語の長さの合計」を足して返す
/// （つまり戻り値は、合計の3倍になる）。読むだけなので RwLock の read を使う。
pub fn total_length_times_three(words: Vec<String>) -> usize {
    let shared = Arc::new(RwLock::new(words));
    let mut handles = Vec::new();

    for _ in 0..3 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let total: usize =
                todo!("read ロックを取り、全単語の長さ（len）の合計を求めてください");
            total
        }));
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// RwLock に書き込みを行い、書き込み後の要素数を返す。
pub fn push_and_count(words: Vec<String>, new_word: &str) -> usize {
    let shared = Arc::new(RwLock::new(words));
    todo!("write ロックを取って new_word を追加し、その後 read ロックで要素数を返してください")
}
