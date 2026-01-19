/*
 * ORIGINAL FILE: raw/buscabinaria.c
 *
 * Binary search program.
 * Using shift instead of division by 2.
 *
 * This Rust version preserves the same procedural structure:
 * explicit indices, explicit loops, and explicit control flow.
 */

fn main() {
    let mut i: i32;
    let mut array: [i32; 25] = [0; 25];
    let mut start: i32 = 0;
    let mut end: i32 = 24;
    let target: i32 = 41; // value to search
    let mut mid: i32;

    // fill array with odd numbers
    print!("[ ");
    for i in 0..25 {
        let element = i * 2 + 1;
        array[i as usize] = element;

        if i == 24 {
            print!("{}]", element);
            break;
        }

        print!("{}, ", element);
    }

    // binary search
    while start <= end {
        // shift instead of /2
        mid = (start + end) >> 1;

        if array[mid as usize] == target {
            println!("\nfound {} at position {}", target, mid);
            return;
        }

        if array[mid as usize] < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    println!("did not find {}", target);
}

