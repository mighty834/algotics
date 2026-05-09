/// # Count Primes
///
/// Given an integer `n`, return the number of prime numbers that are strictly less than `n`.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: n = 10
/// Output: 4
/// Explanation: There are 4 prime numbers less than 10: 2, 3, 5, 7.
/// ```
///
/// ### Example 2:
/// ```text
/// Input: n = 0
/// Output: 0
/// ```
///
/// ### Example 3:
/// ```text
/// Input: n = 1
/// Output: 0
/// ```
///
/// ## Constraints
///
/// - `0 <= n <= 5 * 10^6`
///
/// ## Notes
///
/// - A common efficient approach is the **Sieve of Eratosthenes**,
///   which runs in `O(n log log n)` time.
pub struct Solutions;

impl Solutions {
    /// Counts how many prime numbers are **strictly less** than `n`.
    ///
    /// # Algorithm
    ///
    /// Uses a boolean sieve (Sieve of Eratosthenes):
    ///
    /// - Allocate a `Vec<bool>` of length `n` initialized to `true`.
    /// - Mark indices `0` and `1` as non-prime.
    /// - For each `i` from `2` up to `n - 1`, if `i` is still marked prime, mark its multiples
    ///   (`2i`, `3i`, …) as non-prime.
    /// - Count remaining `true` entries.
    ///
    /// # Arguments
    ///
    /// * `n` — Upper bound (exclusive). Typical constraints allow up to `5_000_000`.
    ///
    /// # Returns
    ///
    /// The number of primes `p` such that `0 <= p < n`.
    ///
    /// # Complexity
    ///
    /// - **Time:** \(O(n \log \log n)\) for the sieve marking work (typical for Eratosthenes).
    /// - **Extra space:** `O(n)` for the boolean sieve.
    ///
    /// # Notes
    ///
    /// For `n <= 2`, this returns `0` (there are no primes less than 2).
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let mut sleve: Vec<bool> = vec![true; n as usize];
        (sleve[0], sleve[1]) = (false, false);

        let nu = n as usize;
        for i in 2..n {
            let iu = i as usize;

            if sleve[iu] {
                let mut j = iu;
                while j < nu {
                    j += iu;

                    if j >= nu {
                        break;
                    }
                    sleve[j] = false;
                }
            }
        }

        sleve.iter().filter(|&x| *x).count() as i32
    }
}

#[cfg(test)]
mod count_primes_tests {
    use super::*;

    macro_rules! test_case {
        ($n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let n: i32 = $n;
                let expected: i32 = $expected;
                let result: i32 = Solutions::count_primes(n);

                assert_eq!(result, expected);
            }
        };
    }

    test_case!(10, 4, test_case_1);
    test_case!(0, 0, test_case_2);
    test_case!(1, 0, test_case_3);
    test_case!(2, 0, test_case_4);
    test_case!(80, 22, test_case_5);
    test_case!(5_000_000, 348513, test_case_6);
}
