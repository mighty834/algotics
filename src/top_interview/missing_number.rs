/// # Missing Number
///
/// Given an array `nums` containing `n` distinct numbers in the range `[0, n]`,
/// return the only number in the range that is missing from the array.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: nums = [3,0,1]
/// Output: 2
///
/// Explanation:
/// n = 3 since there are 3 numbers, so all numbers are in the range [0,3].
/// 2 is missing because it does not appear in the array.
/// ```
///
/// ### Example 2:
/// ```text
/// Input: nums = [0,1]
/// Output: 2
///
/// Explanation:
/// n = 2 since there are 2 numbers, so all numbers are in the range [0,2].
/// 2 is missing because it does not appear in the array.
/// ```
///
/// ### Example 3:
/// ```text
/// Input: nums = [9,6,4,2,3,5,7,0,1]
/// Output: 8
///
/// Explanation:
/// n = 9 since there are 9 numbers, so all numbers are in the range [0,9].
/// 8 is missing because it does not appear in the array.
/// ```
///
/// ## Constraints
///
/// - `n == nums.len()`
/// - `1 <= n <= 10^4`
/// - `0 <= nums[i] <= n`
/// - All numbers in `nums` are unique.
///
/// ## Follow-up
///
/// Can you solve it using:
///
/// - `O(1)` extra space
/// - `O(n)` time complexity
///
/// ## Notes
///
/// Common approaches:
///
/// - Sum formula:
///
/// ```text
/// expected_sum = n * (n + 1) / 2
/// missing = expected_sum - actual_sum
/// ```
///
/// - XOR approach:
///
/// ```text
/// missing = (0 ^ 1 ^ ... ^ n) ^ (nums[0] ^ nums[1] ^ ...)
/// ```
///
/// ## Complexity
///
/// XOR solution:
///
/// - Time: `O(n)`
/// - Space: `O(1)`
pub struct Solutions;

impl Solutions {
    /// Return the one integer in `[0, n]` that does not appear in `nums`.
    ///
    /// Here `n = nums.len()`. The array holds `n` distinct values drawn from `0..=n`,
    /// so exactly one value from that range is missing.
    ///
    /// # Arguments
    ///
    /// * `nums` — Distinct integers in `[0, n]` with `n == nums.len()`.
    ///
    /// # Returns
    ///
    /// The missing value in `[0, n]`.
    ///
    /// # Algorithm
    ///
    /// 1. Insert all elements of `nums` into a [`HashSet`] for `O(1)` membership checks.
    /// 2. Scan `i` from `0` to `n - 1`; return the first `i` not present in the set.
    /// 3. If every value in `0..n` is present, return `n` (the only remaining candidate).
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: nums = [3, 0, 1]
    /// Output: 2
    /// ```
    ///
    /// ```text
    /// Input: nums = [0, 1]
    /// Output: 2
    /// ```
    ///
    /// ```text
    /// Input: nums = [9, 6, 4, 2, 3, 5, 7, 0, 1]
    /// Output: 8
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(n)** to build the set and scan `0..=n`.
    /// - Extra space: **O(n)** for the [`HashSet`].
    ///
    /// # See also
    ///
    /// The module-level notes describe **O(1)** extra-space variants (sum formula, XOR).
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let set: HashSet<i32> = HashSet::from_iter(nums);
        for i in 0..set.len() as i32 {
            if !set.contains(&i) {
                return i;
            }
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod missing_number_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let nums: Vec<i32> = $nums;
                let expected: i32 = $expected;

                let result: i32 = Solutions::missing_number(nums);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![3, 0, 1], 2, test_case_1);
    test_case!(vec![0, 1], 2, test_case_2);
    test_case!(vec![9, 6, 4, 2, 3, 5, 7, 0, 1], 8, test_case_3);
    test_case!(vec![1, 0, 2, 4, 3, 5, 6, 7, 8], 9, test_case_4);
    test_case!(vec![1], 0, test_case_5);
    test_case!(vec![0], 1, test_case_6);
    test_case!(vec![2, 1, 3, 5, 4], 0, test_case_7);
    test_case!(vec![1, 4, 5, 3, 0], 2, test_case_8);
    test_case!(vec![6, 1, 0, 2, 3, 5, 7], 4, test_case_9);
}
