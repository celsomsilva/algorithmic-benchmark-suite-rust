use std::time::Instant;
use algorithmic_benchmark_suite_rust::search::binary_search;

fn main() {
    let size: i32 = 1_000_000;
    let mut array: Vec<i32> = Vec::with_capacity(size as usize);

    for i in 0..size {
        array.push(i * 2 + 1);
    }

    let target = size * 2 - 1;
    let iterations = 1_000_000;

    let start = Instant::now();

    for _ in 0..iterations {
        binary_search::search(&array, target);
    }

    let duration = start.elapsed();

    println!("Binary Search Benchmark");
    println!("Array size   : {}", size);
    println!("Iterations   : {}", iterations);
    println!("Elapsed time : {:?}", duration);
}

