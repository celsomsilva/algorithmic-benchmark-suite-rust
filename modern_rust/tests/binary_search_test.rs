use algorithmic_benchmark_suite_rust::search::binary_search::BinarySearch;

#[test]
fn test_binary_search_found_middle() {
    let array = [1, 3, 5, 7, 9];
    let index = BinarySearch::search(&array, 7);
    assert_eq!(index, 3);
}

#[test]
fn test_binary_search_found_first() {
    let array = [1, 3, 5, 7, 9];
    let index = BinarySearch::search(&array, 1);
    assert_eq!(index, 0);
}

#[test]
fn test_binary_search_found_last() {
    let array = [1, 3, 5, 7, 9];
    let index = BinarySearch::search(&array, 9);
    assert_eq!(index, 4);
}

#[test]
fn test_binary_search_not_found() {
    let array = [1, 3, 5, 7, 9];
    let index = BinarySearch::search(&array, 4);
    assert_eq!(index, -1);
}

