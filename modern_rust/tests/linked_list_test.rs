use algorithmic_benchmark_suite_rust::structures::linked_list::run_example;

#[test]
fn test_linked_list_final_state() {
    let result = run_example();
    assert_eq!(result, vec![9, 6, 2, 10]);
}

