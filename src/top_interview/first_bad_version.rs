/// # First Bad Version
///
/// You are a product manager and currently leading a team to develop a new product.
/// Unfortunately, the latest version of your product fails the quality check.
/// Since each version is developed based on the previous version, all the versions
/// after a bad version are also bad.
///
/// Suppose you have `n` versions `[1, 2, ..., n]` and you want to find out the first
/// bad one, which causes all the following ones to be bad.
///
/// You are given an API `isBadVersion(version)` which returns whether a version is bad.
/// Implement a function to find the first bad version. You should minimize the number
/// of calls to the API.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: n = 5, bad = 4
/// Output: 4
/// Explanation:
/// call isBadVersion(3) -> false
/// call isBadVersion(5) -> true
/// call isBadVersion(4) -> true
/// Then 4 is the first bad version.
/// ```
///
/// ### Example 2:
/// ```text
/// Input: n = 1, bad = 1
/// Output: 1
/// ```
///
/// ## Constraints
///
/// - `1 <= bad <= n <= 2^31 - 1`
///
/// ## Notes
///
/// - The `isBadVersion` API is defined externally.
/// - You should aim to minimize the number of API calls (e.g., using binary search).
pub struct Solutions {
    bad: i32
}

impl Solutions {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, n: i32) -> bool {
        n >= self.bad
    }

    /// Returns the smallest version in `[1, n]` that is bad, assuming versions `[bad, n]` are all
    /// bad and `[1, bad - 1]` are good.
    ///
    /// # Algorithm
    ///
    /// Binary search on the answer range `[1, n]`. Maintain `min` and `max` as ends of the
    /// interval that still contains the first bad version. Let `mid = (min + max) / 2` using
    /// `u32` arithmetic so `min + max` never overflows `i32`.
    ///
    /// - If `isBadVersion(mid)` is true, the first bad version is at most `mid`, so set `max = mid`.
    /// - Otherwise the first bad version is after `mid`, so set `min = mid`.
    ///
    /// The loop stops when only two adjacent candidates remain (`max - 1 == min`) or when
    /// `min == max`. Then return `min` if it is bad, otherwise `max` (exactly one of the two is
    /// the boundary when two candidates remain).
    ///
    /// # Arguments
    ///
    /// * `n` — Number of versions `1..=n`. The stub `isBadVersion` treats versions `>= bad` as
    ///   bad; inputs must satisfy `1 <= bad <= n` for the `bad` value stored on this [`Solutions`].
    ///
    /// # Returns
    ///
    /// The first bad version index, in `1..=n`.
    ///
    /// # Complexity
    ///
    /// `O(log n)` calls to `isBadVersion` in the worst case.
    ///
    /// # Panics
    ///
    /// Does not panic for valid LeetCode inputs. Misconfigured `bad` relative to `n` can yield
    /// incorrect results or panic from arithmetic edge cases; tests keep `bad` in `[1, n]`.
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut min, mut max) = (1, n);

        while min != max && max - 1 != min {
            let mid = (((min as u32) + (max as u32)) / 2) as i32;

            if self.isBadVersion(mid) {
                max = mid;
            } else {
                min = mid;
            }
        }

        if self.isBadVersion(min) { min } else { max }
    }
}

#[cfg(test)]
mod first_bad_version_tests {
    use super::*;

    macro_rules! test_case {
        ($n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let n: i32 = $n;
                let expected: i32 = $expected;
                let solution = Solutions { bad: $expected };

                let result: i32 = solution.first_bad_version(n);
                assert_eq!(expected, result);
            }
        }
    }

    test_case!(5, 4, test_case_1);
    test_case!(1, 1, test_case_2);
    test_case!(1001, 27, test_case_3);
    test_case!(1001, 1001, test_case_4);
    test_case!(i32::MAX, 1, test_case_5);
    test_case!(i32::MAX, i32::MAX, test_case_6);
    test_case!(i32::MAX, i32::MAX / 2, test_case_7);
    test_case!(678934, 246762, test_case_8);
}