use std::sync::{Arc, Mutex, RwLock};
use std::thread;

pub fn parallel_increment(threads: usize, per_thread: u64) -> u64 {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();

    for _ in 0..threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..per_thread {
                let mut guard = c.lock().unwrap();
                *guard += 1;
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    let total = *counter.lock().unwrap();
    total
}

pub fn total_length_times_three(words: Vec<String>) -> usize {
    let shared = Arc::new(RwLock::new(words));
    let mut handles = Vec::new();

    for _ in 0..3 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let guard = s.read().unwrap();
            guard.iter().map(|w| w.len()).sum::<usize>()
        }));
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

pub fn push_and_count(words: Vec<String>, new_word: &str) -> usize {
    let shared = Arc::new(RwLock::new(words));
    shared.write().unwrap().push(new_word.to_string());
    let count = shared.read().unwrap().len();
    count
}
