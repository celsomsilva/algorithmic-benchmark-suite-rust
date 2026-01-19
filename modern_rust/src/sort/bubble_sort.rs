/*
 * ORIGINAL FILE: bubblesort.c
 *
 * Simple bubble sort implementation.
 * Sorts an integer array in ascending order.
 *
 * Educational version (no early exit optimization).
 *
 * This Rust version preserves the same procedural structure:
 * explicit loops, explicit indices, and in-place mutation.
 */
 
// This file contains a standalone Rust translation of a C algorithm.
// Functions may appear unused because they are meant for comparison,
// benchmarking, or standalone execution.
// #![allow(dead_code)]


fn main() {
    let mut values: [i32; 8] = [3, 0, 5, 1, 8, 9, 2, 4];
    let size: i32 = 8;

    print!("ARRAY: [ ");
    print_array(&values, size);
    println!("]");

    bubble_sort(&mut values, size);

    print!("SORTED ARRAY: [ ");
    print_array(&values, size);
    println!("]");
}

/*
 * bubble_sort:
 * Performs an in-place bubble sort on the array.
 */
pub fn bubble_sort(array: &mut [i32; 8], size: i32) {

    let mut temp: i32;

    for _j in 0..size - 1 {  // or for _ in 0..size - 1 because j is irrelevant (it isn't in the loop)
        for i in 0..size - 1 {
            let idx = i as usize;
            if array[idx] > array[idx + 1] {
                temp = array[idx];
                array[idx] = array[idx + 1];
                array[idx + 1] = temp;
            }
        }
    }
}

/*
 * print_array:
 * Prints the elements of the array.
 */
fn print_array(array: &[i32; 8], size: i32) {

    for i in 0..size {
        print!("{} ", array[i as usize]);
    }
}

