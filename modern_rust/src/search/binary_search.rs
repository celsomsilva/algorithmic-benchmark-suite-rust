/*
 * ORIGINAL FILE: raw/buscabinaria.c
 *
 * Binary search program.
 * Using shift instead of division by 2.
 *
 * This Rust version preserves the same procedural structure:
 * explicit indices, explicit loops, and explicit control flow.
 */
 
// This file contains a standalone Rust translation of a C algorithm.
// Functions may appear unused because they are meant for comparison,
// benchmarking, or standalone execution.
//#![allow(dead_code)]



pub fn search(array: &[i32], target: i32) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = array.len() as i32 - 1;
    let mut mid: i32;

    while start <= end {
        mid = (start + end) >> 1;

        if array[mid as usize] == target {
            return mid;
        }

        if array[mid as usize] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    -1
}

fn main() {
    let mut array: [i32; 25] = [0; 25];

    // fill array with odd numbers
    for i in 0..25 {
        array[i as usize] = i * 2 + 1;
    }

    let target = 41;
    let index = search(&array, target);

    if index >= 0 {
        println!("found {} at position {}", target, index);
    } else {
        println!("did not find {}", target);
    }
}

