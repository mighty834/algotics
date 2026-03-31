/// # Rotate Array
/// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
///
/// ## Example 1:
/// ```text
/// Input: nums = [1,2,3,4,5,6,7], k = 3
/// Output: [5,6,7,1,2,3,4]
/// Explanation:
/// rotate 1 steps to the right: [7,1,2,3,4,5,6]
/// rotate 2 steps to the right: [6,7,1,2,3,4,5]
/// rotate 3 steps to the right: [5,6,7,1,2,3,4]
/// ```
///
/// ## Example 2:
/// ```text
/// Input: nums = [-1,-100,3,99], k = 2
/// Output: [3,99,-1,-100]
/// Explanation:
/// rotate 1 steps to the right: [99,-1,-100,3]
/// rotate 2 steps to the right: [3,99,-1,-100]
/// ```
///
/// ## Constraints:
/// * `1 <= nums.length <= 105`
/// * `-231 <= nums[i] <= 231 - 1`
/// * `0 <= k <= 105`
///
/// ## Follow up:
/// * Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
/// * Could you do it in-place with O(1) extra space?
pub struct Solutions;

impl Solutions {
    /// Rotate using the standard library's built‑in `rotate_right`.
    ///
    /// **Idea**
    /// - First reduce `k` modulo `nums.len()` so we never rotate more than the
    ///   length of the array (rotating by `len` is a no‑op).
    /// - Delegate the actual rotation to `Vec::rotate_right`, which performs
    ///   an in‑place rotation with O(1) extra memory.
    ///
    /// **Complexity**
    /// - Time: O(n), where `n = nums.len()`, because each element is moved at most once.
    /// - Space: O(1) additional memory (rotation happens in place).
    pub fn built_in_rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.rotate_right(k);
    }

    /// Same approach as [`built_in_rotate`](Self::built_in_rotate), but converts `k` to `usize` using `try_from`.
    ///
    /// **Idea**
    /// - Use `usize::try_from(k)` to avoid a potentially lossy cast from `i32` to `usize`.
    /// - If `k` is negative or doesn't fit into `usize`, this panics with a clear message.
    /// - After the safe conversion, we again take `k % len` and call `rotate_right`.
    ///
    /// **When to use**
    /// - When you prefer a defensive, explicit check that `k` fits into `usize`,
    ///   instead of silently casting.
    pub fn built_in_rotate_with_safe_usize(nums: &mut Vec<i32>, k: i32) {
        let k: usize = usize::try_from(k).unwrap_or_else(|_| panic!("k is not usize-able!"));
        let k = k % nums.len();
        nums.rotate_right(k);
    }

    /// Rotate by splitting the slice into two parts and stitching them into a new buffer.
    ///
    /// **Idea**
    /// - Compute the split index `m = len - k`.
    /// - Use `split_at(m)` to get the left part `[0..m)` and the right part `[m..len)`.
    /// - Build a new vector `result = right + left`, then copy it back into `nums`.
    ///
    /// **Trade‑offs**
    /// - Time: O(n) to build `result` and clone it back.
    /// - Space: O(n) extra, because we allocate a separate `Vec<i32>` for the rotated data.
    pub fn rotate_by_two_slices(nums: &mut Vec<i32>, k: i32) {
        let k: usize = k.try_into().unwrap_or_else(|_| panic!("k is not usize-able!"));
        let k = k % nums.len();
        let (left, right) = nums.split_at(nums.len() - k);

        let mut result: Vec<i32> = Vec::with_capacity(nums.len());
        result.extend_from_slice(right);
        result.extend_from_slice(left);

        nums.clone_from_slice(&result);
    }

    /// Rotate by cloning once and rebuilding `nums` from two slices.
    ///
    /// **Idea**
    /// - Clone the original vector into `twin`.
    /// - Split `twin` into `left` and `right` at `len - k`.
    /// - Clear `nums` and extend it with `right` followed by `left`.
    ///
    /// This is very explicit and easy to read: we construct the rotated order directly
    /// from two consecutive slices.
    pub fn rotate_by_two_slices_with_clear(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();

        let twin: Vec<i32> = nums.clone();
        let (left, right) = twin.split_at(nums.len() - k);

        nums.clear();
        nums.extend_from_slice(right);
        nums.extend_from_slice(left);
    }

    /// Rotate by appending rotated data to `nums`, then draining the original prefix.
    ///
    /// **Idea**
    /// - Clone `nums` into `twin` and split it into `left` and `right` at `len - k`.
    /// - Reserve enough capacity so `nums` can hold its current contents plus the rotated
    ///   data without reallocating.
    /// - Append `right` then `left` to `nums`, so `nums` becomes `[original | right | left]`.
    /// - `drain(0..twin.len())` removes the original segment, leaving `[right | left]` — the
    ///   rotated result — in place.
    ///
    /// **Trade‑offs**
    /// - Reuses `nums`’ buffer instead of allocating a separate result vector, but
    ///   temporarily doubles capacity and does one clone plus a drain (which moves
    ///   the tail down). Time O(n), extra space O(n) for the clone; buffer may stay
    ///   at 2× capacity until the next realloc.
    pub fn rotate_by_two_slices_with_drain(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.reserve(nums.len() * 2);

        let twin: Vec<i32> = nums.clone();
        let (left, right) = twin.split_at(nums.len() - k);

        nums.extend_from_slice(right);
        nums.extend_from_slice(left);
        nums.drain(0..twin.len());
    }

    /// Naive rotation by repeatedly popping from the end and inserting at the front.
    ///
    /// **Idea**
    /// - Reduce `k` modulo `len`.
    /// - For each step:
    ///   - `pop()` the last element.
    ///   - `insert(0, value)` to push it to the front.
    ///
    /// **Why this is inefficient**
    /// - Each `insert(0, value)` shifts all existing elements to the right, which is O(n).
    /// - Repeating this `k` times gives O(k * n) time in the worst case.
    ///
    /// Good as a first, straightforward solution, but not suitable for large inputs.
    pub fn rotate_brute_force(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        for _ in 0..k {
            let value = nums.pop().unwrap();
            nums.insert(0, value);
        }
    }

    /// Rotate using two explicit slices taken from indexing.
    ///
    /// **Idea**
    /// - Compute `m = len - k` and create two borrowed slices:
    ///   - `left = &nums[..m]`
    ///   - `right = &nums[m..]`
    /// - Build `result = right + left` just like in [`rotate_by_two_slices`](Self::rotate_by_two_slices),
    ///   but using manual slice syntax instead of `split_at`.
    ///
    /// Functionally equivalent to `rotate_by_two_slices`; the difference is mostly style.
    pub fn rotate_by_two_handmade_slices(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let m = nums.len() - k;
        let (left, right) = (&nums[..m], &nums[m..]);

        let mut result: Vec<i32> = Vec::with_capacity(nums.len());
        result.extend_from_slice(right);
        result.extend_from_slice(left);

        nums.clone_from_slice(&result);
    }

    /// Rotate by cloning once and directly extending from two computed slices.
    ///
    /// **Idea**
    /// - Clone `nums` into `twin`.
    /// - Clear `nums`.
    /// - Extend first with the last `k` elements from `twin`, then with the first `len - k`.
    ///
    /// This avoids an intermediate `result` vector and writes directly into `nums`,
    /// at the cost of one `clone()` of the original data.
    pub fn rotate_with_direct_slices_extension(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let twin: Vec<i32> = nums.clone();

        nums.clear();
        nums.extend_from_slice(&twin[twin.len() - k..]);
        nums.extend_from_slice(&twin[..twin.len() - k]);
    }

    /// Rotate by manually pushing elements from two ranges with `for` loops.
    ///
    /// **Idea**
    /// - Clone `nums` into `twin`.
    /// - Clear `nums`.
    /// - Push elements from the tail slice `[l - k..]`, then from the head slice `[..l - k]`.
    ///
    /// This is a very explicit version of the slice‑based solutions, useful for understanding
    /// the rotation logic step by step.
    ///
    /// **Complexity**
    /// - Time: O(n) (each element is read once and written once).
    /// - Space: O(n) extra for the `twin` clone.
    pub fn rotate_with_two_fors(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let l = nums.len();
        let twin = nums.clone();

        nums.clear();
        for value in &twin[l - k..] {
            nums.push(*value);
        }

        for value in &twin[..l - k] {
            nums.push(*value);
        }
    }

    /// Rotate using iterator adapters: `skip`, `chain`, then `collect`.
    ///
    /// **Idea**
    /// - Reduce `k` modulo `len` so we only rotate by the effective amount.
    /// - The rotated order is the last `k` elements followed by the first `len - k`:
    ///   `nums[len - k..]` then `nums[..len - k]`.
    /// - Express that as iterators: [`Iterator::skip`] on `nums` to skip the first
    ///   `len - k` items (yielding the tail), [`Iterator::chain`] with [`Iterator::take`]
    ///   on the start of the slice (the head), [`Iterator::cloned`], and [`Iterator::collect`]
    ///   into a new `Vec`.
    /// - Copy the result back with `clone_from_slice` on the mutable slice of `nums`.
    ///
    /// **Complexity**
    /// - Time: O(n) — one pass over the elements when collecting.
    /// - Space: O(n) for the temporary `result` vector.
    ///
    /// **Note**
    /// - Requires `nums` to be non-empty (same assumption as other helpers here), since
    ///   `k % nums.len()` is undefined for `len == 0`.
    pub fn rotate_with_iter(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();

        let result: Vec<i32> = nums
            .iter()
            .skip(nums.len() - k)
            .chain(nums.iter().take(nums.len() - k))
            .cloned()
            .collect();

        nums.clone_from_slice(&result);
    }
}

#[cfg(test)]
mod rotate_array_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $k: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let mut nums: Vec<i32> = $nums;
                let k: i32 = $k;
                let expected: Vec<i32> = $expected;

                Solutions::built_in_rotate(&mut nums, k);
                assert_eq!(nums, expected, "built_in_rotate solution failed");

                nums = $nums;
                Solutions::built_in_rotate_with_safe_usize(&mut nums, k);
                assert_eq!(nums, expected, "built_in_rotate_with_safe_usize solution failed");

                nums = $nums;
                Solutions::rotate_by_two_slices(&mut nums, k);
                assert_eq!(nums, expected, "rotate_by_two_slices solution failed");

                nums = $nums;
                Solutions::rotate_by_two_slices_with_clear(&mut nums, k);
                assert_eq!(nums, expected, "rotate_by_two_slices_with_clear solution failed");

                nums = $nums;
                Solutions::rotate_by_two_slices_with_drain(&mut nums, k);
                assert_eq!(nums, expected, "rotate_by_two_slices_with_drain solution failed");

                nums = $nums;
                Solutions::rotate_brute_force(&mut nums, k);
                assert_eq!(nums, expected, "rotate_brute_force solution failed");

                nums = $nums;
                Solutions::rotate_by_two_handmade_slices(&mut nums, k);
                assert_eq!(nums, expected, "rotate_by_two_handmade_slices solution failed");

                nums = $nums;
                Solutions::rotate_with_direct_slices_extension(&mut nums, k);
                assert_eq!(nums, expected, "rotate_with_direct_slices_extension solution failed");

                nums = $nums;
                Solutions::rotate_with_two_fors(&mut nums, k);
                assert_eq!(nums, expected, "rotate_with_two_fors solution failed");

                nums = $nums;
                Solutions::rotate_with_iter(&mut nums, k);
                assert_eq!(nums, expected, "rotate_with_iter failed");
            }
        };
        (
            nums=$nums: expr,
            k=$k: expr,
            expected=$expected: expr,
            test_case_name=$fn_name: ident
        ) => {
            test_case!($nums, $k, $expected, $fn_name);
        };
    }

    test_case!(
        nums = vec![1, 2, 3, 4, 5, 6, 7],
        k = 3,
        expected = vec![5, 6, 7, 1, 2, 3, 4],
        test_case_name = test_case_1
    );
    test_case!(vec![1], 13, vec![1], test_case_2);
    test_case!(vec![1, 2, 3, 4], 25, vec![4, 1, 2, 3], test_case_3);
    test_case!(vec![-100, 99, 13, 29], 0, vec![-100, 99, 13, 29], test_case_4);
    test_case!(vec![5, 6, 7, 8, 9], 105, vec![5, 6, 7, 8, 9], test_case_5);
    test_case!(vec![5, 6, 7, 8], 105, vec![8, 5, 6, 7], test_case_6);
    test_case!(vec![1, 2], 105, vec![2, 1], test_case_7);
}
