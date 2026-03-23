use std::iter::chain;

/// Moves all zero values in the array to the end while preserving the
/// relative order of non-zero elements.
///
/// The operation must be performed **in-place**, meaning no additional
/// array or copy of the input should be created.
///
/// # Examples
///
/// ```text
/// Input: nums = [0,1,0,3,12]
/// Output: [1,3,12,0,0]
/// ```
///
/// ```text
/// Input: nums = [0]
/// Output: [0]
/// ```
///
/// # Constraints
///
/// - `1 <= nums.len() <= 10^4`
/// - `-2^31 <= nums[i] <= 2^31 - 1`
///
/// # Requirements
///
/// - Preserve the relative order of non-zero elements
/// - Modify the input array in-place
///
/// # Follow-up
///
/// - Can you minimize the total number of operations performed?
///
/// # Notes
///
/// - The result should not allocate a new array
/// - Only rearrangement of existing elements is allowed
pub struct Solutions;

impl Solutions {
    /// Moves all zeroes to the end of `nums` while keeping non-zero order stable.
    ///
    /// # Algorithm used here
    ///
    /// - Scan the vector with index `cx`.
    /// - When `nums[cx] == 0`, remove that element via `drain(cx..=cx)` and count it.
    /// - When `nums[cx] != 0`, advance `cx`.
    /// - After the scan, append `zero_count` trailing zeroes.
    ///
    /// This preserves the relative order of non-zero elements because only zeroes are
    /// removed during traversal, and all removed zeroes are appended at the end.
    ///
    /// # Complexity
    ///
    /// - Time: worst-case **O(n²)** due to repeated single-element `drain` shifts.
    /// - Extra space: **O(z)** for the appended zero buffer, where `z` is zero count.
    ///
    /// # Note
    ///
    /// A two-pointer overwrite strategy can solve this in **O(n)** time and **O(1)**
    /// extra space. This implementation favors straightforward logic.
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut zero_count, mut cx) = (0, 0);
        while cx < nums.len() {
            if nums[cx] == 0 {
                nums.drain(cx..=cx);
                zero_count += 1;
            } else {
                cx += 1;
            }
        }

        nums.extend_from_slice(&vec![0; zero_count]);
    }

    /// Reorders `nums` by collecting non-zero values first, then zeroes.
    ///
    /// # Why "cheated"
    ///
    /// This version is concise and expressive, but it builds an intermediate vector
    /// (`temp`) instead of performing a strict in-place rearrangement.
    ///
    /// # Algorithm
    ///
    /// - Iterate `nums` and keep all non-zero elements.
    /// - Chain that with another iterator over all zero elements.
    /// - Collect into `temp`, then copy `temp` back into `nums`.
    ///
    /// Relative order is preserved within both groups because iterator traversal
    /// is stable.
    ///
    /// # Complexity
    ///
    /// - Time: **O(n)** for filtering, chaining, collecting, and copying back.
    /// - Extra space: **O(n)** for `temp`.
    ///
    /// # Note
    ///
    /// If the interview requirement is strictly in-place with O(1) extra space,
    /// prefer a two-pointer overwrite/swap strategy.
    pub fn move_zeroes_cheated(nums: &mut Vec<i32>) {
        let temp: Vec<i32> = nums.iter()
            .filter(|&x| *x != 0)
            .chain(nums.iter().filter(|&x| *x == 0))
            .map(|x| *x)
            .collect();

        nums.clone_from_slice(&temp);
    }
}

#[cfg(test)]
mod move_zeroes_tests {

    use super::*;

    macro_rules! test_case {
        ($nums: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let mut nums: Vec<i32> = $nums;
                let expected: Vec<i32> = $expected;

                Solutions::move_zeroes(&mut nums);
                assert_eq!(nums, expected);

                nums = $nums;
                Solutions::move_zeroes_cheated(&mut nums);
                assert_eq!(nums, expected);
            }
        }
    }

    test_case!(vec![1, 0, 2, 0, 3], vec![1, 2, 3, 0, 0], test_case_1);
    test_case!(vec![0], vec![0], test_case_2);
    test_case!(vec![0, 0, 0, 1], vec![1, 0, 0, 0], test_case_3);
    test_case!(vec![1], vec![1], test_case_4);
    test_case!(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], test_case_5);
    test_case!(vec![1, 2, 0, 3, 4, 0], vec![1, 2, 3, 4, 0, 0], test_case_6);
    test_case!(vec![0, 0, 1, 1, 0, 2, 0, 3], vec![1, 1, 2, 3, 0, 0, 0, 0], test_case_7);
    test_case!(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5], vec![1, 2, 3, 4, 5, 0, 0, 0, 0, 0], test_case_8);
}
