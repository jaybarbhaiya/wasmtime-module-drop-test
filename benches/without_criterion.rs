use std::{hint::black_box, time::Instant};

use wasmtime_module_drop_bench::create_module;

const ITERATIONS: usize = 5_000;

fn bench_create_module<F, O>(name: &'static str, mut func: F)
where
    F: FnMut() -> O,
{
    // pre-allocate to_be_dropped items
    let to_be_dropped = black_box((0..ITERATIONS).map(|_| func()).collect::<Vec<_>>());
    let mut output = Vec::with_capacity(ITERATIONS);
    let start = Instant::now();
    output.extend(to_be_dropped.into_iter().map(|i| {
        std::mem::drop(criterion::black_box(i));
    }));
    let time = start.elapsed();
    criterion::black_box(output);
    let time_per_iter = time / ITERATIONS as u32;
    println!("bench {} took {:?} per iter", name, time_per_iter);
}

fn main() {
    bench_create_module("drop module (big) (old)", create_module);
}
