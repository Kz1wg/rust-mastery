use p05_cache::{Cache, Clock, Stats};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// テスト用の時計。advance で好きなだけ時間を進められる。
struct FakeClock(AtomicU64);

impl FakeClock {
    fn new(start: u64) -> Arc<Self> {
        Arc::new(FakeClock(AtomicU64::new(start)))
    }
    fn advance(&self, secs: u64) {
        self.0.fetch_add(secs, Ordering::SeqCst);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> u64 {
        self.0.load(Ordering::SeqCst)
    }
}

#[test]
fn insert_and_get() {
    let clock = FakeClock::new(1000);
    let cache: Cache<String> = Cache::new(60, clock);
    cache.insert("a", "apple".to_string());
    assert_eq!(cache.get("a"), Some("apple".to_string()));
    assert_eq!(cache.get("zzz"), None);
    assert_eq!(cache.stats(), Stats { hits: 1, misses: 1 });
}

/// 1時間待たなくても、時計を進めるだけで期限切れを確かめられる（Lesson 13-1）。
#[test]
fn entries_expire_after_ttl() {
    let clock = FakeClock::new(1000);
    let cache: Cache<u32> = Cache::new(3600, clock.clone());
    cache.insert("k", 1);

    clock.advance(3599);
    assert_eq!(cache.get("k"), Some(1), "まだ期限内");

    clock.advance(1);
    assert_eq!(cache.get("k"), None, "ちょうど期限で切れる");
    assert_eq!(cache.len(), 0);
}

#[test]
fn insert_again_refreshes_expiry() {
    let clock = FakeClock::new(0);
    let cache: Cache<u32> = Cache::new(10, clock.clone());
    cache.insert("k", 1);
    clock.advance(8);
    cache.insert("k", 2);
    clock.advance(8);
    assert_eq!(cache.get("k"), Some(2));
}

#[test]
fn len_counts_only_live_entries() {
    let clock = FakeClock::new(0);
    let cache: Cache<u32> = Cache::new(10, clock.clone());
    cache.insert("old", 1);
    clock.advance(5);
    cache.insert("new", 2);
    clock.advance(6); // old は期限切れ、new はまだ有効
    assert_eq!(cache.len(), 1);
    assert!(!cache.is_empty());
}

#[test]
fn get_or_insert_with_computes_only_when_missing() {
    let clock = FakeClock::new(0);
    let cache: Cache<String> = Cache::new(10, clock);
    let mut calls = 0;
    let v1 = cache.get_or_insert_with("k", || {
        calls += 1;
        "computed".to_string()
    });
    let v2 = cache.get_or_insert_with("k", || {
        calls += 1;
        "again".to_string()
    });
    assert_eq!((v1.as_str(), v2.as_str()), ("computed", "computed"));
    assert_eq!(calls, 1);
}

/// Cache は Arc で包めば、複数のスレッドから共有できる（Send + Sync）。
#[test]
fn cache_is_shared_across_threads() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Cache<String>>();

    let clock = FakeClock::new(0);
    let cache = Arc::new(Cache::<u64>::new(100, clock));
    let handles: Vec<_> = (0..8u64)
        .map(|i| {
            let cache = Arc::clone(&cache);
            std::thread::spawn(move || {
                cache.insert(&format!("key{i}"), i);
                cache.get(&format!("key{i}"))
            })
        })
        .collect();

    for (i, h) in handles.into_iter().enumerate() {
        assert_eq!(h.join().unwrap(), Some(i as u64));
    }
    assert_eq!(cache.len(), 8);
    assert_eq!(cache.stats().hits, 8);
}
