use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasmtime_module_drop_bench::create_module;

pub fn create_module_bench(c: &mut Criterion) {
    c.bench_function("drop wasmtime::Module", |b| {
        b.iter_batched(
            create_module,
            |module| std::mem::drop(black_box(module)),
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000).without_plots();
    targets = create_module_bench
}
criterion_main!(benches);
