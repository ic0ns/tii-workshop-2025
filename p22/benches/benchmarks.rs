use criterion::{Criterion, black_box, criterion_group, criterion_main};
use my_crate::fibonacci_loop;

fn bench_fibonacci_loop(c: &mut Criterion) {
    c.bench_function("fibonacci_loop 20", |b| {
        b.iter(|| fibonacci_loop(black_box(20)))
    });
}

criterion_group!(benches, bench_fibonacci_loop);
criterion_main!(benches);
