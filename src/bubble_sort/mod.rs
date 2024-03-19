use std::clone::Clone;

/// Bubble sort implementation with no optimization.
fn bubble_sort_no_opti<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let n = arr.len();
    let mut arr = arr.to_vec(); // Prefer to_vec() for converting slices to Vec

    let mut nb_comp = 0;

    // Outer loop: continues as long as a swap occurred in the previous pass.
    for _ in 1..n {
        // Inner loop: goes through the array from the end towards the current position.
        for j in 1..n {
            nb_comp += 1;

            if arr[j - 1] > arr[j] {
                // Swap if the current pair is out of order.
                arr.swap(j, j - 1);
            }
        }
    }

    println!("Number of iterations of 'bubble_sort_no_opti': {}", nb_comp);

    arr
}

/// Bubble sort implementation with a better loop.
fn bubble_sort_better_loop<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let n = arr.len();
    let mut arr = arr.to_vec(); // Prefer to_vec() for converting slices to Vec

    let mut nb_comp = 0;

    // Outer loop: continues as long as a swap occurred in the previous pass.
    for i in 1..n {
        // Inner loop: goes through the array from the end towards the current position.
        for j in (i..n).rev() {
            nb_comp += 1;

            if arr[j - 1] > arr[j] {
                // Swap if the current pair is out of order.
                arr.swap(j, j - 1);
            }
        }
    }

    println!("Number of iterations of 'bubble_sort_better_loop': {}", nb_comp);

    arr
}

/// Bubble sort implementation with a better loop and an early exit.
fn bubble_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let n = arr.len();
    let mut arr = arr.to_vec(); // Prefer to_vec() for converting slices to Vec (do a clone)
    let mut swapped; // early exit if the array is already sorted.

    let mut nb_comp = 0;

    // Outer loop: continues as long as a swap occurred in the previous pass.
    for i in 1..n {
        swapped = false; // Reset swapped flag for the current pass.

        // Inner loop: goes through the array from the end towards the current position.
        for j in (i..n).rev() {
            nb_comp += 1;

            if arr[j - 1] > arr[j] {
                // Swap if the current pair is out of order.
                arr.swap(j, j - 1);
                swapped = true; // Indicate that a swap occurred.
            }
        }

        if !swapped {
            break; // If no swaps occurred in the previous pass, the array is sorted.
        }
    }

    println!("Number of iterations of 'bubble_sort': {}", nb_comp);

    arr
}

use crate::test::test_sort_and_check;

pub fn test_bubble_sorts(
    random_array: Vec<u32>,
    sorted_array: Vec<u32>
) {
    println!("🔬 Testing bubble_sort...");
    test_sort_and_check(bubble_sort_no_opti, &random_array, &sorted_array);
    test_sort_and_check(bubble_sort_better_loop, &random_array, &sorted_array);
    test_sort_and_check(bubble_sort, &random_array, &sorted_array);
}
