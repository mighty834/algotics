/// # Shuffle an Array
///
/// Given an integer array `nums`, design an algorithm to randomly shuffle the array.
/// All permutations of the array should be equally likely as a result of the shuffling.
///
/// Implement the `Solution` struct:
///
/// - `Solution::new(nums)` initializes the object with the integer array `nums`.
/// - `reset()` resets the array to its original configuration and returns it.
/// - `shuffle()` returns a random shuffling of the array.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input:
/// ["Solution", "shuffle", "reset", "shuffle"]
/// [[[1, 2, 3]], [], [], []]
///
/// Output:
/// [null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]
///
/// Explanation:
/// Solution solution = new Solution([1, 2, 3]);
/// solution.shuffle(); // returns a random permutation, e.g., [3, 1, 2]
/// solution.reset();   // returns [1, 2, 3]
/// solution.shuffle(); // returns another random permutation, e.g., [1, 3, 2]
/// ```
///
/// ## Constraints
///
/// - `1 <= nums.length <= 50`
/// - `-10^6 <= nums[i] <= 10^6`
/// - All elements of `nums` are unique.
/// - At most `10^4` calls in total will be made to `reset` and `shuffle`.
///
/// ## Notes
///
/// - A correct solution should ensure that each permutation is equally likely
///   (e.g., using the Fisher–Yates shuffle).
use rand::RngExt;

pub trait Solution {
    fn new(nums: Vec<i32>) -> Self
    where
        Self: Sized;
    fn reset(&self) -> Vec<i32>;
    fn shuffle(&self) -> Vec<i32>;
}

macro_rules! solution {
    ($(#[$meta:meta])* $solution_name: ident) => {
        $(#[$meta])*
        pub struct $solution_name {
            origin: Vec<i32>,
        }
    };
}

solution!(
    /// Shuffles by the **Fisher–Yates** algorithm (a.k.a. Knuth shuffle).
    ///
    /// # Idea
    ///
    /// Walk from left to right; for each position `i`, pick a uniformly random index `j` in
    /// `[i, n)` and swap `temp[i]` with `temp[j]`. If the RNG is uniform, this produces a uniform
    /// random permutation over all `n!` orderings.
    ///
    /// # Notes
    ///
    /// Uses `rand` for randomness. This implementation clones the original vector before shuffling
    /// so that `reset()` can always return the original configuration.
    FisherYatesSolution
);
impl Solution for FisherYatesSolution {
    fn new(nums: Vec<i32>) -> Self {
        Self { origin: nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.origin.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut temp = self.origin.clone();
        let mut rng = rand::rng();

        for i in 0..temp.len() {
            let second_i = rng.random_range(i..temp.len());
            temp.swap(i, second_i);
        }

        temp
    }
}

solution!(
    /// Shuffles using the Fisher–Yates swap pattern, but with a tiny custom RNG.
    ///
    /// # Idea
    ///
    /// This follows the same index-selection pattern as Fisher–Yates (pick `j` in `[i, n)` and
    /// swap), but uses a simple linear congruential generator (LCG) seeded with a fixed constant.
    ///
    /// # Important
    ///
    /// - This is **not** cryptographically secure.
    /// - A fixed seed means the “random” shuffle is deterministic across runs.
    /// - Uniformity depends on RNG quality; LCG modulo bias can make the distribution non-uniform.
    ///
    /// It’s included as a “roll your own RNG” variant for learning, not as a recommended approach.
    CustomRundomSolution
);
impl Solution for CustomRundomSolution {
    fn new(nums: Vec<i32>) -> Self {
        Self { origin: nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.origin.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        struct SimpleRng {
            state: u64,
        }

        impl SimpleRng {
            fn new(seed: u64) -> Self {
                Self { state: seed }
            }

            fn next(&mut self) -> u32 {
                // constants from Numerical Recipes
                self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
                (self.state >> 16) as u32
            }

            fn range(&mut self, upper: usize) -> usize {
                (self.next() as usize) % upper
            }
        }

        let mut temp = self.origin.clone();
        let mut rng = SimpleRng::new(123);

        for i in 0..temp.len() {
            let j = i + rng.range(temp.len() - i);
            temp.swap(i, j);
        }

        temp
    }
}

#[cfg(test)]
mod shuffle_an_array_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let solutions: Vec<Box<dyn Solution>> = vec![
                    Box::new(FisherYatesSolution::new($nums)),
                    Box::new(CustomRundomSolution::new($nums)),
                ];

                for solution in solutions {
                    let mut shuffle = solution.shuffle();
                    let mut reset = solution.reset();

                    assert_eq!(reset, $nums);

                    shuffle.sort();
                    reset.sort();
                    assert_eq!(reset, shuffle);
                }
            }
        };
    }

    test_case!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], test_case_1);
    test_case!(vec![2, 3, 1, 4, 5], test_case_2);
    test_case!(vec![1], test_case_3);
    test_case!(vec![100; 50], test_case_4);
    test_case!(vec![100, 110, 120, 130, 140, 150, 160, 170, 180, 190], test_case_5);
}
