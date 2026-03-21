/// # Remove Duplicates from Sorted Array
/// Removes duplicates from a sorted array in-place.
///
/// Given a vector `nums` sorted in non-decreasing order, removes duplicates such
/// that each unique element appears only once. The relative order of the elements
/// is preserved.
///
/// Returns the number of unique elements `k`.
///
/// After the function returns, the first `k` elements of `nums` contain the unique
/// values in sorted order. The remaining elements may contain any values and are
/// not considered.
///
/// # Arguments
///
/// * `nums` - A mutable vector of integers sorted in non-decreasing order.
///
/// # Returns
///
/// The number of unique elements in `nums`.
///
/// # Examples
///
/// ```text
/// let mut nums = vec![1, 1, 2];
/// let k = remove_duplicates(&mut nums);
///
/// assert_eq!(k, 2);
/// assert_eq!(&nums[..k], &[1, 2]);
/// ```
///
/// ```text
/// let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
/// let k = remove_duplicates(&mut nums);
///
/// assert_eq!(k, 5);
/// assert_eq!(&nums[..k], &[0, 1, 2, 3, 4]);
/// ```
///
/// # Constraints
///
/// * `1 <= nums.len() <= 3 * 10^4`
/// * `-100 <= nums[i] <= 100`
/// * `nums` is sorted in non-decreasing order.
pub struct Solutions;

impl Solutions {
    /// Remove duplicates in-place using a single write index (“slow pointer”).
    ///
    /// **Idea**
    /// - Because `nums` is sorted, duplicates are contiguous. Keep the first element
    ///   at index `0`, then walk `i` from `1..len`.
    /// - Whenever `nums[i]` differs from `nums[i - 1]`, it is a new unique value: copy it
    ///   to `nums[cx]` and advance `cx` (the next position to write).
    /// - Values at indices `cx..` are overwritten or ignored; only `nums[..cx]` matters.
    ///
    /// **Returns**
    /// - `cx` as `i32`: the length of the prefix that contains all unique elements in order.
    ///
    /// **Complexity**
    /// - Time: O(n) with O(1) extra space (only `cx` and the loop index).
    pub fn simple_index(nums: &mut Vec<i32>) -> i32 {
        let mut cx: usize = 1;

        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] {
                nums[cx] = nums[i];
                cx += 1;
            }
        }

        return cx as i32;
    }

    /// Collect unique values into a [`BTreeSet`](std::collections::BTreeSet), then copy
    /// them back to the front of `nums`.
    ///
    /// **Idea**
    /// - Insert every element into a [`BTreeSet`](std::collections::BTreeSet), which stores
    ///   **sorted** unique values
    ///   (same order as the original array, since input is already sorted).
    /// - Iterate the set in order and overwrite `nums[0]`, `nums[1]`, … with those values.
    ///
    /// **Returns**
    /// - The number of unique elements (`set.len()` as `i32`), i.e. how many leading
    ///   positions of `nums` are now valid.
    ///
    /// **Complexity**
    /// - Time: O(n log n) for building and iterating the set.
    /// - Space: O(n) for the set.
    ///
    /// **Trade‑offs**
    /// - Simpler mentally than the two-pointer approach, but uses extra memory and does
    ///   not beat O(n) time. Prefer [`simple_index`](Self::simple_index) for interviews
    ///   and large inputs.
    pub fn throw_hash_set(nums: &mut Vec<i32>) -> i32 {
        use std::collections::BTreeSet;

        let set: BTreeSet<i32> = nums.iter().map(|i| *i).collect();
        for (i, v) in set.iter().enumerate() {
            nums[i] = *v;
        }

        return set.len() as i32;
    }
}

#[cfg(test)]
mod remove_duplicates_from_sorted_array_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let mut nums: Vec<i32> = $nums;
                let expected: Vec<i32> = $expected;

                let k: i32 = Solutions::simple_index(&mut nums);
                assert_eq!(k, expected.len().try_into().unwrap_or_else(|_| panic!("BAD expected!")));
                assert_eq!(&nums[..k as usize], &expected);

                nums = $nums;
                let k: i32 = Solutions::throw_hash_set(&mut nums);
                assert_eq!(k, expected.len().try_into().unwrap());
                assert_eq!(&nums[..k as usize], &expected);
            }
        };
    }

    test_case!(vec![1, 1, 1, 2, 2, 3, 3, 4], vec![1, 2, 3, 4], test_case_1);
    test_case!(vec![1], vec![1], test_case_2);
    test_case!(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5], test_case_3);
    test_case!(vec![1, 2, 2, 2, 2, 2], vec![1, 2], test_case_4);
    test_case!(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], vec![1, 2, 3, 4, 5], test_case_5);
    test_case!(vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4], vec![1, 2, 3, 4], test_case_6);
    test_case!(vec![8, 8, 8, 8, 8], vec![8], test_case_7);
}
