/// # Merge Sorted Array
///
/// You are given two integer arrays `nums1` and `nums2`, sorted in non-decreasing order,
/// and two integers `m` and `n`, representing the number of elements in `nums1` and
/// `nums2` respectively.
///
/// Merge `nums1` and `nums2` into a single array sorted in non-decreasing order.
///
/// The final sorted array should not be returned by the function, but instead be stored
/// inside the array `nums1`. To accommodate this, `nums1` has a length of `m + n`,
/// where the first `m` elements denote the elements that should be merged, and the last
/// `n` elements are set to `0` and should be ignored. `nums2` has a length of `n`.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
/// Output: [1,2,2,3,5,6]
/// Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
/// The result of the merge is [1,2,2,3,5,6].
/// ```
///
/// ### Example 2:
/// ```text
/// Input: nums1 = [1], m = 1, nums2 = [], n = 0
/// Output: [1]
/// Explanation: The arrays we are merging are [1] and [].
/// The result of the merge is [1].
/// ```
///
/// ### Example 3:
/// ```text
/// Input: nums1 = [0], m = 0, nums2 = [1], n = 1
/// Output: [1]
/// Explanation: The arrays we are merging are [] and [1].
/// The result of the merge is [1].
/// Note that because m = 0, there are no elements in nums1.
/// The 0 is only there to ensure the merge result can fit in nums1.
/// ```
///
/// ## Constraints
///
/// - `nums1.length == m + n`
/// - `nums2.length == n`
/// - `0 <= m, n <= 200`
/// - `1 <= m + n <= 200`
/// - `-10^9 <= nums1[i], nums2[j] <= 10^9`
///
/// ## Follow-up
///
/// Can you come up with an algorithm that runs in `O(m + n)` time?
pub struct Solutions;

impl Solutions {
    /// Merges the sorted data in `nums2` into the sorted prefix of `nums1`, leaving the full
    /// merged, sorted sequence in `nums1`.
    ///
    /// # Algorithm
    ///
    /// 1. **Drop the tail buffer** — `nums1.drain(m..m + n)` removes the last `n` elements of
    ///    `nums1`, so it temporarily holds only the `m` values of the first array (still sorted).
    /// 2. **Insert from `nums2` in order** — while `nums2` is not empty, take the first
    ///    element (with `remove(0)`), scan `nums1` from the start, and `insert` it at the first
    ///    index where the current `nums1` value is not less than that element (non-decreasing
    ///    order). If the value is larger than all of `nums1`, it is `push`ed; the first time
    ///    that happens, the rest of `nums2` is `append`ed in one step because the remainder is
    ///    already sorted and all not less than the tail of `nums1`.
    ///
    /// This is a straightforward merge-by-insertion. It is **not** the usual O(m + n) *in-place
    /// from the end* three-pointer approach; it can do more work per element because `insert` and
    /// `remove(0)` shift memory in a `Vec`.
    ///
    /// # Arguments
    ///
    /// * `nums1` — On input, length `m + n`: the first `m` entries are the first sorted array,
    ///   the last `n` are scratch (e.g. zeros). On output, the first `m + n` entries are the
    ///   full merged array in non-decreasing order. The vector may grow if the implementation
    ///   appends; callers should size `nums1` to at least `m + n` as in the problem statement.
    /// * `m` — Count of real elements from the first array in `nums1` before the merge.
    /// * `nums2` — The second sorted array, length `n` on entry. Elements are removed from the
    ///   front with `remove(0)` until one value is larger than every element of `nums1`; then
    ///   that value is `push`ed and the rest of `nums2` is moved onto `nums1` with `append`.
    /// * `n` — Count of elements in `nums2`.
    ///
    /// # Complexity
    ///
    /// * **Time** — In the worst case, each `insert` / `remove(0)` is linear in the current
    ///   length of the vector, so the cost is much higher than the optimal O(m + n) merge.
    /// * **Space** — Uses only the vectors’ existing allocation plus temporary shifting (no
    ///   separate merge buffer).
    ///
    /// # Panics
    ///
    /// May panic or corrupt results if `nums1.len()` is not `m + n`, if `nums2.len()` does not
    /// match `n`, or if `m` / `n` are negative or ranges are invalid for `drain`.
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.drain(m as usize..(m + n) as usize);

        'outer: while nums2.len() != 0 {
            let val = nums2.remove(0);
            let mut cx = 0;

            while cx < nums1.len() {
                if val <= nums1[cx] {
                    nums1.insert(cx, val);
                    continue 'outer;
                }

                cx += 1;
            }

            nums1.push(val);
            nums1.append(nums2);
            break;
        }
    }
}

#[cfg(test)]
mod merge_sorted_array_tests {
    use super::*;

    macro_rules! test_case {
        ($nums1: expr, $m: expr, $nums2: expr, $n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let expected: Vec<i32> = $expected;

                let mut nums1: Vec<i32> = $nums1;
                let mut nums2: Vec<i32> = $nums2;
                let m: i32 = $m;
                let n: i32 = $n;

                Solutions::merge(&mut nums1, m, &mut nums2, n);
                assert_eq!(nums1, expected);
            }
        };
    }

    test_case!(
        vec![1, 2, 0, 0],
        2,
        vec![3, 4],
        2,
        vec![1, 2, 3, 4],
        test_case_1
    );

    test_case!(
        vec![1, 2, 3, 0, 0, 0],
        3,
        vec![2, 5, 6],
        3,
        vec![1, 2, 2, 3, 5, 6],
        test_case_2
    );

    test_case!(
        vec![5, 6, 7, 0, 0, 0],
        3,
        vec![1, 2, 3],
        3,
        vec![1, 2, 3, 5, 6, 7],
        test_case_3
    );

    test_case!(vec![0], 0, vec![1], 1, vec![1], test_case_4);
    test_case!(vec![1], 1, vec![], 0, vec![1], test_case_5);

    test_case!(
        vec![1, 4, 8, 0, 0],
        3,
        vec![2, 7],
        2,
        vec![1, 2, 4, 7, 8],
        test_case_6
    );

    test_case!(
        vec![1, 2, 3, 4, 6, 7, 8, 9, 0],
        8,
        vec![5],
        1,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        test_case_7
    );

    test_case!(
        vec![4, 7, 9, 14, 20, 21, 0, 0, 0],
        6,
        vec![1, 12, 22],
        3,
        vec![1, 4, 7, 9, 12, 14, 20, 21, 22],
        test_case_8
    );

    test_case!(
        vec![1, 2, 3, 0, 0, 0],
        3,
        vec![4, 5, 6],
        3,
        vec![1, 2, 3, 4, 5, 6],
        test_case_9
    );
}
