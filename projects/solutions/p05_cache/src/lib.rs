//! P5: 有効期限つきのキャッシュ。複数のスレッドから共有して使える。
//!
//! - 中身は Mutex で守る（Lesson 10-2）。Cache を Arc で包めば、スレッド間で共有できる
//! - ヒット数・ミス数は AtomicU64 で数える（ロックを取らずに増やせる）
//! - 現在時刻は Clock trait で外から受け取る（Lesson 13-1）。テストでは時計を自由に進められる

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// 現在時刻（秒）を返すもの。スレッド間で共有するので Send + Sync を要求する。
pub trait Clock: Send + Sync {
    fn now(&self) -> u64;
}

/// 本物の時計。UNIX 時刻（秒）を返す。
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// ヒット数とミス数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub hits: u64,
    pub misses: u64,
}

struct Entry<V> {
    value: V,
    expires_at: u64,
}

/// 有効期限つきのキャッシュ。
pub struct Cache<V> {
    entries: Mutex<HashMap<String, Entry<V>>>,
    ttl_secs: u64,
    clock: Arc<dyn Clock>,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl<V: Clone> Cache<V> {
    /// ttl_secs 秒で期限切れになるキャッシュを作る。
    pub fn new(ttl_secs: u64, clock: Arc<dyn Clock>) -> Self {
        Cache {
            entries: Mutex::new(HashMap::new()),
            ttl_secs,
            clock,
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    /// 値を入れる（同じキーがあれば上書きし、期限も延ばす）。
    pub fn insert(&self, key: &str, value: V) {
        let expires_at = self.clock.now() + self.ttl_secs;
        self.entries
            .lock()
            .unwrap()
            .insert(key.to_string(), Entry { value, expires_at });
    }

    /// 値を取り出す（クローンを返す）。
    /// 期限切れ（now >= expires_at）なら、その項目を消して None。
    /// 見つかれば hits、見つからなければ（期限切れも含めて）misses を1増やす。
    pub fn get(&self, key: &str) -> Option<V> {
        let now = self.clock.now();
        let mut entries = self.entries.lock().unwrap();
        match entries.get(key) {
            Some(entry) if now < entry.expires_at => {
                self.hits.fetch_add(1, Ordering::Relaxed);
                Some(entry.value.clone())
            }
            Some(_) => {
                entries.remove(key);
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            }
            None => {
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            }
        }
    }

    /// 値があればそれを返し、無ければ make() で作って入れてから返す。
    ///
    /// 注意（設計の問い）: make() を呼んでいる間、ロックを持ち続けるかどうか。
    /// この実装では、get と insert を別々に呼ぶ（make の間はロックを持たない）。
    /// そのため、同時に同じキーを要求されると make() が2回呼ばれることがある。
    pub fn get_or_insert_with<F: FnOnce() -> V>(&self, key: &str, make: F) -> V {
        if let Some(value) = self.get(key) {
            return value;
        }
        let value = make();
        self.insert(key, value.clone());
        value
    }

    /// 期限切れでない項目の数。
    pub fn len(&self) -> usize {
        let now = self.clock.now();
        let entries = self.entries.lock().unwrap();
        entries.values().filter(|e| now < e.expires_at).count()
    }

    /// 期限切れでない項目が1つも無ければ true。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// これまでのヒット数とミス数。
    pub fn stats(&self) -> Stats {
        Stats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
        }
    }
}
