use criterion::{black_box, criterion_group, criterion_main, Criterion};
use project::cache::PersistentCache;
use project::traits::Cache;

fn cache_benchmark(c: &mut Criterion) {
    c.bench_function("insert", |b| {
        let mut cache = PersistentCache::new(100);
        b.iter(|| {
            cache.insert(black_box(1), black_box(1));
        });
    });

    c.bench_function("get", |b| {
        let mut cache = PersistentCache::new(100);
        cache.insert(1, 1);
        b.iter(|| {
            cache.get(&1);
        });
    });
}

criterion_group!(benches, cache_benchmark);
criterion_main!(benches);