/// Checks whether the input array contains any duplicate elements.
///
/// Given a vector `nums`, returns `true` if any value appears at least twice,
/// and `false` if all elements are distinct.
///
/// # Arguments
///
/// * `nums` - A vector of integers.
///
/// # Returns
///
/// `true` if any element appears more than once, otherwise `false`.
///
/// # Examples
///
/// ```text
/// let nums = vec![1, 2, 3, 1];
/// assert_eq!(contains_duplicate(nums), true);
/// ```
///
/// ```text
/// let nums = vec![1, 2, 3, 4];
/// assert_eq!(contains_duplicate(nums), false);
/// ```
///
/// ```text
/// let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
/// assert_eq!(contains_duplicate(nums), true);
/// ```
///
/// # Explanation
///
/// Returns `true` if at least one value appears multiple times in the input.
/// Otherwise, returns `false`.
///
/// # Constraints
///
/// * `1 <= nums.len() <= 10^5`
/// * `-10^9 <= nums[i] <= 10^9`
pub struct Solutions;

impl Solutions {
    /// Brute-force duplicate check: compare every pair of indices.
    ///
    /// **Idea**
    /// For each index `i`, scan all `j > i`. If `nums[i] == nums[j]`, a duplicate exists.
    ///
    /// **Complexity**
    /// - Time: **O(n²)** in the worst case (nested loops).
    /// - Extra space: **O(1)** — no auxiliary structure beyond the input vector.
    ///
    /// **When to use**
    /// Useful as a baseline or when extra memory is forbidden; for large `n`, prefer
    /// [`contains_duplicate_hashing`](Self::contains_duplicate_hashing) or
    /// [`contains_duplicate_hashing_step`](Self::contains_duplicate_hashing_step).
    pub fn contains_duplicate_slow(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }

        return false;
    }

    /// Duplicate check via [`HashSet`](std::collections::HashSet): compare length with unique count.
    ///
    /// **Idea**
    /// Insert all elements into a set. A [`HashSet`](std::collections::HashSet) keeps only distinct values. If
    /// `nums.len() != set.len()`, some value appeared more than once (at least one
    /// duplicate). Otherwise all elements are unique.
    ///
    /// **Complexity**
    /// - Time: **O(n)** average for building the set (each insert is O(1) amortized).
    /// - Extra space: **O(n)** for the set in the worst case.
    ///
    /// **Note**
    /// Consumes `nums` when iterating into the set (`into_iter()`).
    pub fn contains_duplicate_hashing(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let nums_len: usize = nums.len();

        let set: HashSet<i32> = nums.into_iter().collect();
        return nums_len != set.len();
    }

    /// Duplicate check with a single pass over [`HashSet`](std::collections::HashSet) inserts.
    ///
    /// **Idea**
    /// Walk the array and insert each `nums[i]` into a set. After processing `i + 1`
    /// elements, if all were unique, the set has exactly `i + 1` entries. If
    /// `set.len() != i + 1`, an insert did not grow the set — the value was already
    /// present — so we return `true` immediately.
    ///
    /// This is equivalent to [`contains_duplicate_hashing`](Self::contains_duplicate_hashing)
    /// but can **short-circuit** as soon as the first duplicate is seen.
    ///
    /// **Complexity**
    /// - Time: **O(n)** average; best case **O(1)** when the first duplicate is at the start
    ///   of a long tail of repeats (stops early).
    /// - Extra space: **O(k)** where `k` is the number of distinct values seen before a
    ///   duplicate (at most **O(n)**).
    pub fn contains_duplicate_hashing_step(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();

        for i in 0..nums.len() {
            set.insert(nums[i]);
            if i + 1 != set.len() {
                return true;
            }
        }

        return false;
    }

    /// Duplicate check with an explicit [`HashSet::contains`](std::collections::HashSet::contains)
    /// before each [`HashSet::insert`](std::collections::HashSet::insert).
    ///
    /// **Idea**
    /// Maintain a set of values seen so far. For each `nums[i]`, if the set already
    /// holds that value, we have a duplicate and return `true`. Otherwise insert it
    /// and continue.
    ///
    /// **Relation to [`contains_duplicate_hashing_step`](Self::contains_duplicate_hashing_step)**
    /// Both are single-pass and short-circuit on the first duplicate. That variant relies
    /// on `set.len()` vs the number of elements processed; this one asks “have we seen
    /// this value?” directly. You could also use only `insert` and test its return value
    /// (`false` means the value was already present), avoiding the extra `contains` lookup.
    ///
    /// **Complexity**
    /// - Time: **O(n)** average (each `contains` / `insert` is O(1) amortized).
    /// - Extra space: **O(n)** for the set in the worst case (all distinct).
    pub fn contains_duplicate_hashing_contains(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();

        for i in 0..nums.len() {
            if set.contains(&nums[i]) {
                return true;
            }
            set.insert(nums[i]);
        }

        return false;
    }

    /// Duplicate check with a **direct-address bit table** over the assumed value range.
    ///
    /// **Idea**
    /// Treat each possible integer in a fixed window as its own slot. Map `val` to
    /// `index = val + 10⁹` so values in about `[-10⁹, 10⁹]` map to `0 .. 2·10⁹ + 1`.
    /// Use a [`Vec<bool>`]: `false` = not seen, `true` = already seen; second hit ⇒ duplicate.
    ///
    /// **Complexity**
    /// - Time: **O(n)** over `n = nums.len()` (one pass, O(1) indexed access per step).
    /// - Space: **O(V)** for `V = 2·10⁹ + 1` booleans allocated up front — proportional to the
    ///   **value domain width**, not to how many distinct numbers appear in `nums`.
    ///
    /// # Warning — allocation size
    ///
    /// This builds `vec![false; 2 * 10⁹ + 1]`, i.e. **billions** of `bool` elements (gigabytes of
    /// memory). It will usually **fail at allocation** or be impractical on typical hardware.
    /// Use it to illustrate *bounded-domain* indexing; for real workloads prefer
    /// [`contains_duplicate_hashing`](Self::contains_duplicate_hashing) or the other hash-set
    /// variants on this type.
    ///
    /// # Correctness note
    ///
    /// Indices are computed with [`i32::pow`] and [`u32`] casts; values outside the intended
    /// `[-10⁹, 10⁹]` band can **wrap or be out of range** — this matches a “LeetCode-style”
    /// constraint table, not arbitrary `i32` in general.
    pub fn contains_duplicate_big_vec(nums: Vec<i32>) -> bool {
        let mut big_vec: Vec<bool> = vec![false; (10_u32.pow(9) * 2 + 1) as usize];
        let get_index: fn(i32) -> u32 = |val: i32| (val + 10_i32.pow(9)) as u32;

        for i in 0..nums.len() {
            if big_vec[get_index(nums[i]) as usize] {
                return true;
            }

            big_vec[get_index(nums[i]) as usize] = true;
        }

        return false;
    }
}

#[cfg(test)]
mod contains_duplicate_tests {
    use super::*;

    macro_rules! test_case {
        ($nums: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let nums: Vec<i32> = $nums;
                let expected: bool = $expected;

                let result: bool = Solutions::contains_duplicate_slow(nums);
                assert_eq!(result, expected);

                let nums = $nums;
                let result = Solutions::contains_duplicate_hashing(nums);
                assert_eq!(result, expected);

                let nums = $nums;
                let result = Solutions::contains_duplicate_hashing_step(nums);
                assert_eq!(result, expected);

                let nums = $nums;
                let result = Solutions::contains_duplicate_hashing_contains(nums);
                assert_eq!(result, expected);

                let nums = $nums;
                let result = Solutions::contains_duplicate_big_vec(nums);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![1, 2, 3, 4, 5], false, test_case_01);
    test_case!(vec![1, 2, 3, 4, 5, 1], true, test_case_02);
    test_case!(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], true, test_case_03);
    test_case!(vec![1], false, test_case_04);
    test_case!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], false, test_case_05);
    test_case!(vec![-1, -2, -3, -4, -5], false, test_case_06);
    test_case!(vec![-1, -2, -3, -4, -5, -6, -3], true, test_case_07);
    test_case!(vec![-100, 100, 200, -200, 300, -300, 1], false, test_case_08);
    test_case!(vec![100, 100], true, test_case_09);
    test_case!(vec![1, 2, 3, 4, 4], true, test_case_10);
}
