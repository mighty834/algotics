/// Finds two indices such that the corresponding elements add up to a target value.
///
/// Given an integer array `nums` and an integer `target`, returns the indices
/// of the two numbers such that they sum to `target`.
///
/// Each input is guaranteed to have exactly one solution, and the same element
/// cannot be used twice.
///
/// The result can be returned in any order.
///
/// # Examples
///
/// ```text
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: nums[0] + nums[1] == 9
/// ```
///
/// ```text
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
/// ```
///
/// ```text
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
/// ```
///
/// # Constraints
///
/// - `2 <= nums.len() <= 10^4`
/// - `-10^9 <= nums[i] <= 10^9`
/// - `-10^9 <= target <= 10^9`
/// - Exactly one valid answer exists
///
/// # Requirements
///
/// - Do not use the same element twice
/// - Return indices, not values
///
/// # Follow-up
///
/// - Can you design an algorithm with time complexity better than `O(n^2)`?
pub struct Solutions;

impl Solutions {
    /// Returns indices of two elements whose sum equals `target`.
    ///
    /// # Strategy used in this implementation
    ///
    /// This method intentionally reuses
    /// [`intersection_of_two_arrays_2::Solutions::intersect`](crate::top_interview::intersection_of_two_arrays_2::Solutions::intersect):
    ///
    /// 1. Build a complement array `dark_twin`, where each element is `target - nums[i]`.
    /// 2. Intersect `nums` with `dark_twin` to obtain candidate values that can participate
    ///    in a valid pair.
    /// 3. Search pairs inside that candidate list and, once a matching value-pair is found,
    ///    map back to original positions in `nums` using:
    ///    - `position(...)` for the first index,
    ///    - `rposition(...)` for the second index (helps with duplicate values).
    ///
    /// # Return value
    ///
    /// - Returns a vector with two indices `[i, j]` such that `nums[i] + nums[j] == target`.
    /// - Returns an empty vector if no pair is found.
    ///
    /// # Complexity
    ///
    /// This reuse-oriented approach is not optimal:
    ///
    /// - Building complement: `O(n)`
    /// - Intersection call (sort + two pointers): `O(n log n)`
    /// - Pair scan inside intersection: up to `O(k^2)` where `k = intersection.len()`
    /// - Index lookups (`position` / `rposition`) are linear scans
    ///
    /// A standard hash-map two-sum solution can achieve `O(n)` time and `O(n)` space.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use crate::top_interview::intersection_of_two_arrays_2::Solutions as IntersectionSolutions;

        let dark_twin: Vec<i32> = nums.iter().map(|value| target - *value).collect();
        let intersection: Vec<i32> = IntersectionSolutions::intersect(nums.clone(), dark_twin);

        for i in 0..intersection.len() {
            for j in (i + 1)..intersection.len() {
                if intersection[i] + intersection[j] == target {
                    return vec![
                        nums.iter().position(|&v| v == intersection[i]).unwrap() as i32,
                        nums.iter().rposition(|&v| v == intersection[j]).unwrap() as i32,
                    ];
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod two_sum_tests {

    use super::*;

    macro_rules! test_case {
        ($nums: expr, $target: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let nums: Vec<i32> = $nums;
                let target: i32 = $target;
                let mut expected: Vec<i32> = $expected;
                expected.sort();

                let mut result: Vec<i32> = Solutions::two_sum(nums, target);
                result.sort();

                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![0, 0, 0, 0, 1, 1], 2, vec![4, 5], test_case_1);
    test_case!(vec![0, 0], 0, vec![0, 1], test_case_2);
    test_case!(vec![1, 2, 100, 3, 4, 5, 200], 300, vec![2, 6], test_case_3);
    test_case!(vec![-100, -80, 0, 100, 200, 1], 1, vec![2, 5], test_case_4);
    test_case!(vec![-100, -1, 200, 1000, 1, -200], -300, vec![0, 5], test_case_5);
    test_case!(vec![90, 100, 110, 120, 130, 140], 200, vec![0, 2], test_case_6);
}
