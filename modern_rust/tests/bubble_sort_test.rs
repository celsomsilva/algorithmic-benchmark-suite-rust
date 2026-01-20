use algorithmic_benchmark_suite_rust::sort::bubble_sort::bubble_sort;

#[test]
fn test_bubble_sort_basic() {
    let mut array: [i32; 8] = [3, 0, 5, 1, 8, 9, 2, 4];
    let size = 8;

    bubble_sort(&mut array, size);

    assert_eq!(array, [0, 1, 2, 3, 4, 5, 8, 9]);
}

#[test]
fn test_bubble_sort_already_sorted() {
    let mut array: [i32; 8] = [0, 1, 2, 3, 4, 5, 8, 9];
    let size = 8;

    bubble_sort(&mut array, size);

    assert_eq!(array, [0, 1, 2, 3, 4, 5, 8, 9]);
}

#[test]
fn test_bubble_sort_reverse_order() {
    let mut array: [i32; 8] = [9, 8, 5, 4, 3, 2, 1, 0];
    let size = 8;

    bubble_sort(&mut array, size);

    assert_eq!(array, [0, 1, 2, 3, 4, 5, 8, 9]);
}

