use async_std::task;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("async-std");

    let mut inputs = vec![];
    for i in (100..901).step_by(100) {
        inputs.push(i);
    }
    for i in (1000..10001).step_by(1000) {
        inputs.push(i);
    }

    for i in inputs {
        group.bench_with_input(BenchmarkId::new("async-std", i), &i, |b, i| {
            b.iter(|| {
                task::block_on(asyncstd_sieve::sieve(*i));
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
