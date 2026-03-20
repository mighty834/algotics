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
