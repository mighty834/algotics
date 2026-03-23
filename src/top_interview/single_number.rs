/// Finds the element that appears only once in the array.
///
/// Given a non-empty vector `nums`, every element appears exactly twice except
/// for one element which appears only once. This function returns that single element.
///
/// The solution must run in linear time `O(n)` and use constant extra space `O(1)`.
///
/// # Arguments
///
/// * `nums` - A vector of integers where every element appears twice except one.
///
/// # Returns
///
/// The element that appears only once.
///
/// # Examples
///
/// ```text
/// let nums = vec![2, 2, 1];
/// assert_eq!(single_number(nums), 1);
/// ```
///
/// ```text
/// let nums = vec![4, 1, 2, 1, 2];
/// assert_eq!(single_number(nums), 4);
/// ```
///
/// ```text
/// let nums = vec![1];
/// assert_eq!(single_number(nums), 1);
/// ```
///
/// # Explanation
///
/// Each element appears twice except for one. Using bitwise XOR:
///
/// - `a ^ a = 0`
/// - `a ^ 0 = a`
///
/// Applying XOR across all elements cancels out duplicates and leaves the unique element.
///
/// # Constraints
///
/// * `1 <= nums.len() <= 3 * 10^4`
/// * `-3 * 10^4 <= nums[i] <= 3 * 10^4`
/// * Exactly one element appears once; all others appear twice.
pub struct Solutions;

impl Solutions {
    /// Returns the value that appears exactly once using a toggle-set strategy.
    ///
    /// **How this implementation works**
    /// - Keep a `HashSet<i32>` named `result`.
    /// - For each `value`:
    ///   - If insertion succeeds, this is the first time we see it.
    ///   - If insertion fails, the value was already present, so remove it.
    /// - After processing all values, only the single non-duplicated value remains in the set.
    ///
    /// This works because values that appear twice are added then removed, while the unique
    /// value is added once and never removed.
    ///
    /// # Complexity
    ///
    /// - Time: `O(n)` average (hash insert/remove are amortized `O(1)`).
    /// - Extra space: `O(n)` in the worst case.
    ///
    /// # Panics
    ///
    /// Panics with `"BAD nums for input!"` if the input does not contain exactly one
    /// unpaired element (i.e., the final set is empty).
    ///
    /// # Note
    ///
    /// A bitwise XOR approach can achieve `O(1)` extra space for this problem. This
    /// implementation prioritizes clarity over strict constant-space usage.
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut result: HashSet<i32> = HashSet::new();

        for value in nums {
            if !result.insert(value) {
                result.remove(&value);
            }
        }

        *result.iter().next().unwrap_or_else(|| panic!("BAD nums for input!"))
    }
}

#[cfg(test)]
mod single_number_tests {
    use crate::top_interview::single_number::Solutions;

    macro_rules! test_case {
        ($nums: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let nums: Vec<i32> = $nums;
                let expected: i32 = $expected;

                let result = Solutions::single_number(nums);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![1, 1, 2, 2, 3, 3, 4, 4, 5], 5, test_case_1);
    test_case!(vec![1, 2, 3, 4, 1, 2, 3], 4, test_case_2);
    test_case!(vec![1, 2, 2, 1, 3, 4, 4, 3, 5, 6, 5], 6, test_case_3);
    test_case!(vec![1], 1, test_case_4);
    test_case!(vec![9, 8, 7, 6, 5, 6, 7, 8, 9], 5, test_case_5);
    test_case!(vec![-100, -200, -300, -200, -100], -300, test_case_6);
}