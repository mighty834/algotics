/// # Climbing Stairs
///
/// You are climbing a staircase. It takes `n` steps to reach the top.
///
/// Each time you can either climb 1 or 2 steps. In how many distinct ways
/// can you climb to the top?
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: n = 2
/// Output: 2
/// Explanation:
/// There are two ways to climb to the top:
/// 1. 1 step + 1 step
/// 2. 2 steps
/// ```
///
/// ### Example 2:
/// ```text
/// Input: n = 3
/// Output: 3
/// Explanation:
/// There are three ways to climb to the top:
/// 1. 1 step + 1 step + 1 step
/// 2. 1 step + 2 steps
/// 3. 2 steps + 1 step
/// ```
///
/// ## Constraints
///
/// - `1 <= n <= 45`
pub struct Solutions;

impl Solutions {
    /// Returns the number of distinct ways to reach the top of a staircase of `n` steps if each
    /// move can be **1** or **2** steps.
    ///
    /// # Mathematics
    ///
    /// Let `W(k)` be the number of ways to stand on step `k` (1-based). To reach step `k` you
    /// came from step `k - 1` or `k - 2`, so `W(k) = W(k - 1) + W(k - 2)` for `k >= 3`, with
    /// `W(1) = 1` and `W(2) = 2`. That is the Fibonacci recurrence shifted by one index compared
    /// to the usual `(1, 1, 2, 3, …)` sequence.
    ///
    /// # Algorithm
    ///
    /// Iterate with two scalars `(a, b)` holding successive values of the recurrence. Initially
    /// `(a, b) = (1, 1)`, matching `W(1)` and the prior term needed for the update. For each of
    /// the `n - 1` iterations in `1..n`, assign `(a, b) = (b, a + b)`. The result for `n` steps is
    /// `b` after the loop (for `n = 1` the range is empty and `b` stays `1`).
    ///
    /// # Arguments
    ///
    /// * `n` — Number of steps to the top. Expected in `1..=45` for typical problem constraints;
    ///   values outside that range are not validated and may overflow `i32` for large `n`.
    ///
    /// # Returns
    ///
    /// The count of distinct orderings of 1- and 2-step moves that sum to `n`.
    ///
    /// # Complexity
    ///
    /// * **Time:** `O(n)`
    /// * **Extra space:** `O(1)`
    ///
    /// # Panics
    ///
    /// Never panics for normal inputs. For `n <= 0`, the loop may not run and the returned value
    /// is not meaningful relative to the problem statement (callers should pass `n >= 1`).
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        for _ in 1..n {
            (a, b) = (b, a + b);
        }

        b
    }
}

#[cfg(test)]
mod climbing_stairs_tests {
    use super::*;

    macro_rules! test_case {
        ($n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let n: i32 = $n;
                let expected: i32 = $expected;

                let result: i32 = Solutions::climb_stairs(n);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(1, 1, test_case_1);
    test_case!(2, 2, test_case_2);
    test_case!(3, 3, test_case_3);
    test_case!(4, 5, test_case_4);
    test_case!(5, 8, test_case_5);
    test_case!(6, 13, test_case_6);
    test_case!(7, 21, test_case_7);
    test_case!(8, 34, test_case_8);
}
