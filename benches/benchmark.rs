use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::cache::PersistentCache;
use project::traits::Cache;

fn cache_benchmark(c: &mut Criterion) {
    c.bench_function("insert", |b| {
        let mut cache = PersistentCache::new(100);
        b.iter(|| {
            cache.insert(black_box("key".to_string()), black_box("value".to_string()));
        });
    });

    c.bench_function("get", |b| {
        let mut cache = PersistentCache::new(100);
        cache.insert("key".to_string(), "value".to_string());
        b.iter(|| {
            cache.get(&"key".to_string());
        });
    });

    c.bench_function("update", |b| {
        let mut cache = PersistentCache::new(100);
        cache.insert("key".to_string(), "value".to_string());
        b.iter(|| {
            cache.insert(black_box("key".to_string()), black_box("new_value".to_string()));
        });
    });

    c.bench_function("eviction", |b| {
        let mut cache = PersistentCache::new(2);
        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());
        b.iter(|| {
            cache.insert(black_box("key3".to_string()), black_box("value3".to_string()));
        });
    });

    c.bench_function("persist", |b| {
        let mut cache = PersistentCache::new(100);
        cache.insert("key".to_string(), "value".to_string());
        b.iter(|| {
            cache.persist("benchmark_cache.json").unwrap();
        });
    });

    c.bench_function("load", |b| {
        let mut cache = PersistentCache::new(100);
        cache.insert("key".to_string(), "value".to_string());
        cache.persist("benchmark_cache.json").unwrap();
        b.iter(|| {
            let mut loaded_cache = PersistentCache::<String, String>::new(100);
            loaded_cache.load("benchmark_cache.json").unwrap();
        });
    });
}

criterion_group!(benches, cache_benchmark);
criterion_main!(benches);