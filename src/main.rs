use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use bubble_sort::test_bubble_sorts;


mod bubble_sort;
mod test;

fn test_sorts(seed: u64) {
    // Create a seeded RNG
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Generate a random array using the seeded RNG
    let random_array: Vec<u32> = (0..1000).map(|_| rng.gen_range(0..100000)).collect();

    // Create a sorted version of the random array for comparison
    let mut sorted_array = random_array.clone();
    sorted_array.sort();

    // test sort functions
    test_bubble_sorts(random_array, sorted_array);
}


fn main() {
    let seed = 42;

    test_sorts(seed);
}

