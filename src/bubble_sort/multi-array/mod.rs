fn multiarray_swarp<A, B>(i: usize, j: usize, arr_1: &mut Vec<A>, arr_2: &mut Vec<B>) {
    arr_1.swap(i, j);
    arr_2.swap(i, j);
}

/// First naive implementation of the bubble sort algorithm.
pub fn multiarray_bubble_sort_v1(names: Vec<String>, heights: Vec<i32>) -> (Vec<String>, Vec<i32>) {
    let mut names = names;
    let mut heights = heights;
    let mut nb_comp = 0;
    
    for _ in 1..names.len() {
        for j in 1..(names.len()) {
            nb_comp += 1;
            if heights[j-1] < heights[j] {
                multiarray_swarp(j-1, j, &mut names, &mut heights);
            }
        }
    }

    println!("Number of iterations: {}", nb_comp);

    (names, heights)
}

/// Modified bubble sort using a kind of "selection sort" approach.
pub fn multiarray_bubble_sort_v2(names: Vec<String>, heights: Vec<i32>) -> (Vec<String>, Vec<i32>) {
    let mut names = names;
    let mut heights = heights;
    let mut nb_comp = 0;
    
    for i in 1..names.len() {
        for j in 0..i {
            nb_comp += 1;
            if heights[j] < heights[i] {
                multiarray_swarp(i, j, &mut names, &mut heights);
            }
        }
    }

    println!("Number of iterations: {}", nb_comp);

    (names, heights)
}

pub fn multiarray_bubble_sort_v4(names: Vec<String>, heights: Vec<i32>) -> (Vec<String>, Vec<i32>) {
    let mut names = names;
    let mut heights = heights;
    let mut nb_comp = 0;

    let mut swapped = false;
    
    for i in 1..names.len() {
        swapped = false;
        for j in 0..i {
            nb_comp += 1;
            if heights[j] < heights[i] {
                multiarray_swarp(i, j, &mut names, &mut heights);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    println!("Number of iterations: {}", nb_comp);

    (names, heights)
}

pub fn multiarray_bubble_sort_v5(names: Vec<String>, heights: Vec<i32>) -> (Vec<String>, Vec<i32>) {
    let mut names = names;
    let mut heights = heights;
    let mut nb_comp = 0;

    let mut swapped = false;
    
    for i in 1..names.len() {
        swapped = false;
        for j in ((i)..names.len()).rev() {
            nb_comp += 1;
            if heights[j-1] < heights[j] {
                // WARN: not i an j but j-1 and j
                multiarray_swarp(j-1, j, &mut names, &mut heights);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    println!("Number of iterations: {}", nb_comp);

    (names, heights)
}

pub fn multiarray_bubble_sort_v3(names: Vec<String>, heights: Vec<i32>) -> (Vec<String>, Vec<i32>) {
    let mut names = names;
    let mut heights = heights;
    let mut nb_comp = 0;

    let mut swapped = false;
    
    for _ in 1..names.len() {
        swapped = false;
        for j in 1..(names.len()) {
            nb_comp += 1;
            if heights[j-1] < heights[j] {
                multiarray_swarp(j-1, j, &mut names, &mut heights);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    println!("Number of iterations: {}", nb_comp);

    (names, heights)
}



fn sort_and_print_with_version(
    names: &Vec<String>, 
    heights: &Vec<i32>, 
    version: usize,
    sort_func: fn(Vec<String>, Vec<i32>) -> (Vec<String>, Vec<i32>)
) {
    let (sorted_names, sorted_weights) = sort_func(names.clone(), heights.clone());
    println!("Sorted names v{}: [", version);
    for i in 0..names.len() {
        println!("    {} (w: {})", sorted_names[i], sorted_weights[i]);
    }
    println!("]");
    
}

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut swapped = true;

    // Outer loop: continues as long as a swap occurred in the previous pass.
    for i in 1..n {
        if !swapped {
            break; // If no swaps occurred in the previous pass, the array is sorted.
        }
        swapped = false; // Reset swapped flag for the current pass.

        // Inner loop: goes through the array from the end towards the current position.
        for j in (i..n).rev() {
            if arr[j] < arr[j - 1] {
                // Swap if the current pair is out of order.
                arr.swap(j, j - 1);
                swapped = true; // Indicate that a swap occurred.
            }
        }
        // At this point, arr[0..i] is in its final position.
    }
}

fn test_multiarray() {
    let names = vec!["John".to_string(), "Alex".to_string(), "Bob".to_string(), "Anna".to_string(), "Alice".to_string()];
    let heights = vec![3, 2, 5, 7, 1];

    sort_and_print_with_version(&names, &heights, 1, multiarray_bubble_sort_v1);
    sort_and_print_with_version(&names, &heights, 2, multiarray_bubble_sort_v2);
    sort_and_print_with_version(&names, &heights, 3, multiarray_bubble_sort_v3);
    sort_and_print_with_version(&names, &heights, 4, multiarray_bubble_sort_v4);
    sort_and_print_with_version(&names, &heights, 5, multiarray_bubble_sort_v5);
}

