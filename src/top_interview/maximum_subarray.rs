/// # Maximum Subarray
///
/// Given an integer array `nums`, find the subarray with the largest sum,
/// and return its sum.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
/// Output: 6
/// Explanation: The subarray [4,-1,2,1] has the largest sum 6.
/// ```
///
/// ### Example 2:
/// ```text
/// Input: nums = [1]
/// Output: 1
/// Explanation: The subarray [1] has the largest sum 1.
/// ```
///
/// ### Example 3:
/// ```text
/// Input: nums = [5,4,-1,7,8]
/// Output: 23
/// Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
/// ```
///
/// ## Constraints
///
/// - `1 <= nums.length <= 10^5`
/// - `-10^4 <= nums[i] <= 10^4`
///
/// ## Follow-up
///
/// If you have figured out the `O(n)` solution, try coding another solution
/// using the divide and conquer approach, which is more subtle.
pub struct Solutions;

impl Solutions {
    /// Returns the maximum possible sum of a **non-empty** contiguous subarray of `nums`.
    ///
    /// # Algorithm
    ///
    /// This is a one-pass dynamic programming approach in the spirit of Kadane’s algorithm:
    ///
    /// - `curr` tracks the best subarray sum that **ends at the previous index** (a “best ending
    ///   here” value).
    /// - `max` tracks the best subarray sum seen anywhere so far.
    ///
    /// For each new element `x = nums[i]`, the best subarray that ends at `i` is either:
    ///
    /// - start fresh at `x`, or
    /// - extend the previous `curr` by adding `x`.
    ///
    /// The implementation updates `max` using `curr + x` vs `x`, and then updates `curr` to the
    /// corresponding “best ending at `i`” value (with a small branch based on the sign of `x`).
    ///
    /// # Arguments
    ///
    /// * `nums` — Input array. Must be non-empty (typical constraints guarantee this).
    ///
    /// # Returns
    ///
    /// The maximum subarray sum.
    ///
    /// # Complexity
    ///
    /// - **Time:** `O(n)` for `n = nums.len()`
    /// - **Extra space:** `O(1)`
    ///
    /// # Panics
    ///
    /// Panics if `nums` is empty, because the implementation reads `nums[0]` to initialize the
    /// running values.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut max, mut curr) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            max = max.max(curr + nums[i]).max(nums[i]);

            if nums[i] > 0 {
                curr = (curr + nums[i]).max(nums[i]);
            } else {
                curr += nums[i];
            }
        }

        max
    }
}

#[cfg(test)]
mod maximum_subarray_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let nums: Vec<i32> = $nums;
                let expected: i32 = $expected;

                let result: i32 = Solutions::max_sub_array(nums);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6, test_case_1);
    test_case!(vec![1], 1, test_case_2);
    test_case!(vec![5, 4, -1, 7, 8], 23, test_case_3);
    test_case!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 45, test_case_4);
    test_case!(vec![-1, -2, -3, -4, -5, -6, -7, -8, -9], -1, test_case_5);
    test_case!(vec![5, -5, 6, -6, 7, -7, 1, -8, 8, -9, 9], 9, test_case_6);
    test_case!(vec![0, 0, 0, 1, 0, 0, 0], 1, test_case_7);
}
