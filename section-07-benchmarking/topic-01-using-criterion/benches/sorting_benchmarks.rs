// Bring the sorting algorithms into scope
use topic_01_using_criterion::{sort_algo_1, sort_algo_2};

use criterion::{criterion_group, criterion_main, Criterion};

// This function accepts an instance of the criterion struct
fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![20, 18, 10, 6, 11, 45, 91, 4, 80, 100];

    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_2(&mut numbers));
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);


