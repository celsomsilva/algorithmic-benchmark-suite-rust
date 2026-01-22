use std::time::Instant;
use algorithmic_benchmark_suite_rust::sort::bubble_sort::bubble_sort;

fn main() {
    let iterations = 10_000;
    let original: [i32; 8] = [3, 0, 5, 1, 8, 9, 2, 4];

    let start = Instant::now();

    for _ in 0..iterations {
        let mut array = original;
        bubble_sort(&mut array, 8);
    }

    let duration = start.elapsed();

    println!("Bubble Sort Benchmark");
    println!("Array size   : 8");
    println!("Iterations   : {}", iterations);
    println!("Elapsed time : {:?}", duration);
}

