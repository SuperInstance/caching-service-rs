# caching-service-rs

Rust port of [caching-service](https://github.com/SuperInstance/caching-service) — generic in-memory cache.

## Features

- **LRU eviction**: least recently used items evicted when capacity reached
- **TTL expiration**: optional per-cache or per-entry time-to-live
- **Stats tracking**: hits, misses, evictions, hit rate
- **Zero dependencies**

## Usage

```rust
use caching_service::Cache;
use std::time::Duration;

let mut cache = Cache::new(100);
cache.put("key", "value");
assert_eq!(cache.get(&"key"), Some("value"));

// With TTL
let mut ttl_cache = Cache::with_ttl(100, Duration::from_secs(60));
ttl_cache.put("temp", "data");

// Stats
let stats = cache.stats();
println!("Hit rate: {:.1}%", stats.hit_rate() * 100.0);
```

## License

MIT
