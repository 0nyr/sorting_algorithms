pub fn test_sort_and_check(
    sort_func: fn(&[u32]) -> Vec<u32>,
    arr_to_sort: &[u32],
    expected_result: &[u32]
) {
    
    // Use the sorting function on the random array
    let should_be_sorted = sort_func(arr_to_sort);
    

    // Assert equality of the sorted arrays. If they differ, print a detailed error message.
    assert_eq!(should_be_sorted, expected_result, 
        "🔴 The array was not sorted correctly. Expected: {:?}, got: {:?}",
        &expected_result[..std::cmp::min(10, expected_result.len())],
        &should_be_sorted[..std::cmp::min(10, should_be_sorted.len())]
    );

    // Print a success message, displaying only up to the first 10 elements of the sorted array.
    println!("🟢 Test passed.");
}
