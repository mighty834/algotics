/// Returns the intersection of two integer arrays.
///
/// Each element in the result must appear as many times as it shows in both
/// arrays. The result may be returned in any order.
///
/// # Examples
///
/// ```text
/// Input: nums1 = [1,2,2,1], nums2 = [2,2]
/// Output: [2,2]
/// ```
///
/// ```text
/// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
/// Output: [4,9]
/// Explanation: [9,4] is also accepted.
/// ```
///
/// # Constraints
///
/// - `1 <= nums1.len(), nums2.len() <= 1000`
/// - `0 <= nums1[i], nums2[i] <= 1000`
///
/// # Follow-up
///
/// - What if the given arrays are already sorted? How would you optimize your algorithm?
/// - What if `nums1` is much nums1 than `nums2`? Which algorithm is better?
/// - What if elements of `nums2` are stored on disk, and memory is limited such
///   that you cannot load all elements into memory at once?
pub struct Solutions;

impl Solutions {
    /// Returns the multiset intersection of `nums1` and `nums2`.
    ///
    /// Each value appears in the output as many times as it appears in **both**
    /// arrays (i.e. `min(count_in_nums1, count_in_nums2)`).
    ///
    /// # Algorithm (sort + two pointers)
    ///
    /// 1. Sort both arrays.
    /// 2. Walk them with indices `nums1_i` and `nums2_i`:
    ///    - If values are equal, push to result and advance both pointers.
    ///    - Otherwise advance the pointer pointing to the smaller value.
    /// 3. Stop when either pointer reaches the end.
    ///
    /// This naturally handles duplicates because equal values are consumed one-by-one.
    ///
    /// # Complexity
    ///
    /// - Time: `O(n log n + m log m)` for sorting, plus `O(n + m)` scan.
    /// - Extra space: `O(1)` auxiliary for the scan (excluding output), plus sorting costs.
    ///
    /// # Notes
    ///
    /// - Output order is based on sorted traversal (the problem allows any order).
    /// - This implementation clones the input vectors before sorting.
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1: Vec<i32> = nums1.clone();
        let mut nums2: Vec<i32> = nums2.clone();
        nums1.sort();
        nums2.sort();

        let (mut nums1_i, mut nums2_i) = (0_usize, 0_usize);
        let mut result: Vec<i32> = Vec::with_capacity(nums1.len().min(nums2.len()));

        while nums1_i < nums1.len() && nums2_i < nums2.len() {
            if nums1[nums1_i] == nums2[nums2_i] {
                result.push(nums1[nums1_i]);
                nums1_i += 1;
                nums2_i += 1;
            } else {
                if nums1[nums1_i] > nums2[nums2_i] {
                    nums2_i += 1;
                } else {
                    nums1_i += 1;
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod intersection_of_two_arrays_tests {

    use super::*;

    macro_rules! test_case {
        ($nums1: expr, $nums2: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let nums1: Vec<i32> = $nums1;
                let nums2: Vec<i32> = $nums2;
                let mut expected: Vec<i32> = $expected;
                expected.sort();

                let mut result: Vec<i32> = Solutions::intersect(nums1, nums2);
                result.sort();

                assert_eq!(result, expected);
            }
        }
    }

    test_case!(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 6, 7], vec![3, 4, 5], test_case_1);
    test_case!(vec![1, 1, 1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], test_case_2);
    test_case!(vec![1], vec![1], vec![1], test_case_3);
    test_case!(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![], test_case_4);
    test_case!(vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3], test_case_5);
    test_case!(vec![1, 2, 3, 4], vec![4, 5, 6, 7], vec![4], test_case_6);
    test_case!(vec![1, 2, 2, 2, 2, 3, 4, 5, 5, 6], vec![2, 2, 5, 6], vec![2, 2, 5, 6], test_case_7);
    test_case!(vec![1, 1, 2, 2, 3, 3, 4, 4], vec![1, 2, 2, 3, 4, 4, 9], vec![1, 2, 2, 3, 4, 4], test_case_8);
    test_case!(vec![5, 99, 1, 1, 5, 99, 11, 4, 11], vec![11, 8, 5, 99, 4, 1, 90, 1], vec![5, 99, 1, 1, 11, 4], test_case_9);
}