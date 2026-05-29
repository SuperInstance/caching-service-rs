# caching-service-rs

Generic in-memory cache with LRU eviction and optional TTL expiration. Zero dependencies.

## What This Gives You

- **LRU eviction**: when capacity is reached, the least recently used entry is evicted automatically
- **TTL expiration**: set a per-cache or per-entry time-to-live; expired entries are pruned on access
- **Stats tracking**: hits, misses, evictions, and hit rate — know how your cache is performing
- **Generic keys and values**: works with any `K: Hash + Eq + Clone` and any `V`
- **Zero dependencies**: pure Rust, no external crates

## Quick Start

```rust
use caching_service::Cache;
use std::time::Duration;

// Basic LRU cache with 100-item capacity
let mut cache = Cache::new(100);
cache.put("user:1", "Alice");
cache.put("user:2", "Bob");

assert_eq!(cache.get(&"user:1"), Some("Alice"));

// Overwrite updates the value and resets last-accessed time
cache.put("user:1", "Alice Updated");
assert_eq!(cache.get(&"user:1"), Some("Alice Updated"));

// With TTL: entries expire after 60 seconds
let mut ttl_cache = Cache::with_ttl(100, Duration::from_secs(60));
ttl_cache.put("session:abc", "token_data");

// Check stats
let stats = cache.stats();
println!("Hit rate: {:.1}%", stats.hit_rate() * 100.0);
println!("Size: {}/{}", stats.size, stats.capacity);
```

## API Reference

### Cache

```rust
impl<K: Hash + Eq + Clone, V> Cache<K, V> {
    pub fn new(capacity: usize) -> Self;
    pub fn with_ttl(capacity: usize, ttl: Duration) -> Self;
    pub fn get(&mut self, key: &K) -> Option<&V>;
    pub fn put(&mut self, key: K, value: V);
    pub fn remove(&mut self, key: &K) -> Option<V>;
    pub fn contains(&self, key: &K) -> bool;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn clear(&mut self);
    pub fn stats(&self) -> CacheStats;
}
```

### CacheStats

```rust
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub size: usize,
    pub capacity: usize,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64;  // hits / (hits + misses)
}
```

## How It Fits

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem. Used by:

- **agent-identity-rs** — cache trust verification results to avoid re-checking
- **bid-engine-rs** — cache auction results for quick lookup
- **agent-handshake-rs** — cache capability negotiation results

## Testing

**9 tests** covering basic get/put, LRU eviction at capacity, TTL expiration, stats accuracy, overwrite behavior, and remove operations.

## Installation

```toml
# Cargo.toml
[dependencies]
caching-service = { git = "https://github.com/SuperInstance/caching-service-rs" }
```

Requires Rust 2021 edition. No external dependencies.
