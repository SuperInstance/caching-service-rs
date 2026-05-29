# caching-service-rs

Generic in-memory LRU cache with TTL expiration, stats tracking, and zero dependencies.

## What This Gives You

- **LRU eviction** — Oldest unused entries evicted automatically when capacity is reached
- **Per-entry TTL** — Optional time-to-live at cache or individual entry level
- **Hit rate tracking** — Hits, misses, evictions, and computed hit rate
- **O(1) operations** — `get`, `put`, and eviction all run in constant time
- **Zero dependencies** — Pure Rust, `std` only

## Quick Start

```rust
use caching_service::Cache;
use std::time::Duration;

// Basic cache with 100-entry capacity
let mut cache = Cache::new(100);
cache.put("key", "value");
assert_eq!(cache.get(&"key"), Some("value"));

// TTL cache — entries expire after 60 seconds
let mut ttl_cache = Cache::with_ttl(100, Duration::from_secs(60));
ttl_cache.put("temp", "data");
// ... 61 seconds later ...
assert_eq!(ttl_cache.get(&"temp"), None);

// Stats
let stats = cache.stats();
println!("Hit rate: {:.1}%", stats.hit_rate() * 100.0);
println!("Hits: {}, Misses: {}, Evictions: {}", stats.hits, stats.misses, stats.evictions);
```

## API Reference

### `Cache<K, V>`

| Method | Description |
|--------|-------------|
| `Cache::new(capacity)` | Create LRU cache with given capacity |
| `Cache::with_ttl(capacity, duration)` | Create cache with default TTL for all entries |
| `cache.put(key, value)` | Insert an entry (evicts LRU if at capacity) |
| `cache.put_with_ttl(key, value, duration)` | Insert with per-entry TTL |
| `cache.get(&key)` | Get value, refreshing LRU position |
| `cache.remove(&key)` | Remove an entry |
| `cache.len()` | Current entry count |
| `cache.stats()` | `CacheStats` with hits, misses, evictions, hit rate |
| `cache.clear()` | Remove all entries |

### `CacheStats`

| Field | Description |
|-------|-------------|
| `hits` | Successful lookups |
| `misses` | Failed lookups |
| `evictions` | Entries evicted by LRU |
| `hit_rate()` | `hits / (hits + misses)` as `f64` |

## How It Fits

- **[cocapn-health-rs](https://github.com/SuperInstance/cocapn-health-rs)** — Caches health check results to avoid re-pinging healthy services
- **[flux-index-rs](https://github.com/SuperInstance/flux-index-rs)** — Caches frequent search results for sub-millisecond queries
- **[capability-spec-rs](https://github.com/SuperInstance/capability-spec-rs)** — Caches parsed capability schemas

## Testing

9 tests covering basic operations, TTL expiration, LRU eviction order, stats accuracy, and edge cases.

```bash
cargo test
```

## Installation

```toml
# Cargo.toml
[dependencies]
caching-service = { git = "https://github.com/SuperInstance/caching-service-rs" }
```

Or clone and build:

```bash
git clone https://github.com/SuperInstance/caching-service-rs.git
cd caching-service-rs
cargo build
```

Requires Rust 1.70+. No external dependencies.

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance) ecosystem.
