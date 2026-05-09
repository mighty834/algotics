/// # House Robber
///
/// You are a professional robber planning to rob houses along a street.
/// Each house has a certain amount of money stashed. The only constraint
/// stopping you from robbing each of them is that adjacent houses have
/// security systems connected, and it will automatically contact the police
/// if two adjacent houses are broken into on the same night.
///
/// Given an integer array `nums` representing the amount of money of each house,
/// return the maximum amount of money you can rob tonight without alerting the police.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: nums = [1,2,3,1]
/// Output: 4
/// Explanation:
/// Rob house 1 (money = 1) and then rob house 3 (money = 3).
/// Total amount you can rob = 1 + 3 = 4.
/// ```
///
/// ### Example 2:
/// ```text
/// Input: nums = [2,7,9,3,1]
/// Output: 12
/// Explanation:
/// Rob house 1 (money = 2), rob house 3 (money = 9), and rob house 5 (money = 1).
/// Total amount you can rob = 2 + 9 + 1 = 12.
/// ```
///
/// ## Constraints
///
/// - `1 <= nums.length <= 100`
/// - `0 <= nums[i] <= 400`
pub struct Solutions;

impl Solutions {
    /// Returns the maximum amount of money that can be robbed without robbing two adjacent houses.
    ///
    /// This is the classic “House Robber” dynamic programming problem: at each house you either
    /// **skip** it (keep the best total so far) or **rob** it (add its value to the best total up
    /// to two houses back).
    ///
    /// # Recurrence
    ///
    /// Let `dp[i]` be the maximum amount robbable from the prefix `nums[0..=i]`. Then:
    ///
    /// `dp[i] = max(dp[i - 1], dp[i - 2] + nums[i])`
    ///
    /// with `dp[0] = nums[0]` and `dp[1] = max(nums[0], nums[1])`.
    ///
    /// # Implementation notes
    ///
    /// The solution keeps only two rolling states instead of an entire `dp` array:
    ///
    /// - `prev` corresponds to the best total from two steps back (roughly `dp[i-2]`)
    /// - `curr` corresponds to the best total so far (roughly `dp[i-1]` / `dp[i]`)
    ///
    /// On each iteration it decides whether taking `nums[i]` plus the `prev` state beats keeping
    /// `curr`. The code uses a small in-place swap/update sequence to advance these rolling values
    /// without allocating extra memory.
    ///
    /// # Arguments
    ///
    /// * `nums` — Money in each house. Must be non-empty (typical constraints guarantee this).
    ///
    /// # Returns
    ///
    /// The maximum amount that can be robbed without choosing adjacent indices.
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
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev, mut curr) = (0, nums[0]);
        for i in 1..nums.len() {
            if nums[i] + prev > curr {
                curr = curr + prev;
                prev = curr - prev;
                curr = curr - prev;

                curr = nums[i] + curr;
            } else {
                prev = curr;
            }
        }

        curr
    }
}

#[cfg(test)]
mod house_robber_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let nums: Vec<i32> = $nums;
                let expected: i32 = $expected;
                let result: i32 = Solutions::rob(nums);

                assert_eq!(result, expected);
            }
        }
    }

    test_case!(vec![1, 2, 3, 1], 4, test_case_1);
    test_case!(vec![2, 7, 9, 3, 1], 12, test_case_2);
    test_case!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 25, test_case_3);
    test_case!(vec![0], 0, test_case_4);
    test_case!(vec![1, 1, 1, 1, 1], 3, test_case_5);
    test_case!(vec![1, 1], 1, test_case_6);
    test_case!(vec![0, 10, 0, 20, 100], 110, test_case_7);
    test_case!(vec![100, 200, 100, 1, 0, 0, 0], 201, test_case_8);
    test_case!(vec![99, 200, 100, 0, 1, 0, 0], 201, test_case_9);
    test_case!(vec![0, 0, 0, 0, 0, 100, 200, 100, 0, 0, 0, 0, 0], 200, test_case_10);
}