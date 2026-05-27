//! Generic in-memory cache with LRU eviction and TTL expiration.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cache entry with optional TTL.
#[derive(Debug, Clone)]
struct Entry<V> {
    value: V,
    inserted: Instant,
    ttl: Option<Duration>,
    last_accessed: Instant,
    access_count: u64,
}

impl<V> Entry<V> {
    fn new(value: V, ttl: Option<Duration>) -> Self {
        let now = Instant::now();
        Self {
            value,
            inserted: now,
            ttl,
            last_accessed: now,
            access_count: 1,
        }
    }

    fn is_expired(&self) -> bool {
        self.ttl.is_some_and(|dur| self.inserted.elapsed() > dur)
    }

    fn touch(&mut self) {
        self.last_accessed = Instant::now();
        self.access_count += 1;
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Copy, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size: usize,
    pub capacity: usize,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}

/// Generic LRU cache with optional TTL.
#[derive(Debug, Clone)]
pub struct Cache<K, V> {
    entries: HashMap<K, Entry<V>>,
    capacity: usize,
    default_ttl: Option<Duration>,
    stats: CacheStats,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            capacity,
            default_ttl: None,
            stats: CacheStats {
                capacity,
                ..Default::default()
            },
        }
    }

    pub fn with_ttl(capacity: usize, ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            capacity,
            default_ttl: Some(ttl),
            stats: CacheStats {
                capacity,
                ..Default::default()
            },
        }
    }

    /// Insert a key-value pair.
    pub fn put(&mut self, key: K, value: V) {
        self.put_with_ttl(key, value, self.default_ttl);
    }

    /// Insert with explicit TTL.
    pub fn put_with_ttl(&mut self, key: K, value: V, ttl: Option<Duration>) {
        if self.entries.len() >= self.capacity && !self.entries.contains_key(&key) {
            self.evict_lru();
        }
        self.entries.insert(key, Entry::new(value, ttl));
        self.stats.size = self.entries.len();
    }

    /// Get a value, returning None if expired or missing.
    pub fn get(&mut self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let expired = self.entries.get(key).is_some_and(|e| e.is_expired());
        if expired {
            self.entries.remove(key);
            self.stats.misses += 1;
            self.stats.size = self.entries.len();
            return None;
        }
        if let Some(entry) = self.entries.get_mut(key) {
            entry.touch();
            self.stats.hits += 1;
            Some(entry.value.clone())
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// Check if a key exists and is not expired.
    pub fn contains(&self, key: &K) -> bool {
        self.entries.get(key).is_some_and(|e| !e.is_expired())
    }

    /// Remove a key.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let result = self.entries.remove(key).map(|e| e.value);
        self.stats.size = self.entries.len();
        result
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.stats.size = 0;
    }

    /// Purge all expired entries.
    pub fn purge_expired(&mut self) -> usize {
        let keys: Vec<K> = self
            .entries
            .iter()
            .filter(|(_, e)| e.is_expired())
            .map(|(k, _)| k.clone())
            .collect();
        let count = keys.len();
        for key in keys {
            self.entries.remove(&key);
        }
        self.stats.size = self.entries.len();
        count
    }

    fn evict_lru(&mut self) {
        // First try to evict expired entries
        let expired: Option<K> = self
            .entries
            .iter()
            .find(|(_, e)| e.is_expired())
            .map(|(k, _)| k.clone());
        if let Some(key) = expired {
            self.entries.remove(&key);
            self.stats.evictions += 1;
            return;
        }

        // Then evict least recently accessed
        let lru_key = self
            .entries
            .iter()
            .min_by_key(|(_, e)| e.last_accessed)
            .map(|(k, _)| k.clone());
        if let Some(key) = lru_key {
            self.entries.remove(&key);
            self.stats.evictions += 1;
        }
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        let mut s = self.stats;
        s.size = self.entries.len();
        s
    }

    /// Current number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_put_get() {
        let mut cache: Cache<&str, i32> = Cache::new(10);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(1));
        assert_eq!(cache.get(&"b"), Some(2));
        assert_eq!(cache.get(&"c"), None);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache: Cache<&str, i32> = Cache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3); // should evict "a" (LRU)
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(2));
        assert_eq!(cache.get(&"c"), Some(3));
    }

    #[test]
    fn test_overwrite() {
        let mut cache: Cache<&str, i32> = Cache::new(10);
        cache.put("a", 1);
        cache.put("a", 42);
        assert_eq!(cache.get(&"a"), Some(42));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut cache: Cache<&str, i32> = Cache::new(10);
        cache.put("a", 1);
        assert_eq!(cache.remove(&"a"), Some(1));
        assert_eq!(cache.get(&"a"), None);
    }

    #[test]
    fn test_clear() {
        let mut cache: Cache<&str, i32> = Cache::new(10);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_stats() {
        let mut cache: Cache<&str, i32> = Cache::new(10);
        cache.put("a", 1);
        let _ = cache.get(&"a"); // hit
        let _ = cache.get(&"b"); // miss
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_rate() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_contains() {
        let mut cache: Cache<&str, i32> = Cache::new(10);
        cache.put("a", 1);
        assert!(cache.contains(&"a"));
        assert!(!cache.contains(&"b"));
    }

    #[test]
    fn test_ttl_expiration() {
        let mut cache: Cache<&str, i32> = Cache::with_ttl(10, Duration::from_millis(10));
        cache.put("a", 1);
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(cache.get(&"a"), None);
    }

    #[test]
    fn test_purge_expired() {
        let mut cache: Cache<&str, i32> = Cache::with_ttl(10, Duration::from_millis(10));
        cache.put("a", 1);
        cache.put("b", 2);
        std::thread::sleep(Duration::from_millis(50));
        let purged = cache.purge_expired();
        assert_eq!(purged, 2);
        assert!(cache.is_empty());
    }
}
