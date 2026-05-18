/// # Number of 1 Bits
///
/// Given a positive integer `n`, return the number of set bits in its binary
/// representation (also known as the Hamming weight).
///
/// A set bit is a bit with value `1`.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: n = 11
/// Output: 3
///
/// Explanation:
/// Binary representation: 1011
/// Number of set bits: 3
/// ```
///
/// ### Example 2:
/// ```text
/// Input: n = 128
/// Output: 1
///
/// Explanation:
/// Binary representation: 10000000
/// Number of set bits: 1
/// ```
///
/// ### Example 3:
/// ```text
/// Input: n = 2147483645
/// Output: 30
///
/// Explanation:
/// Binary representation:
/// 1111111111111111111111111111101
///
/// Number of set bits: 30
/// ```
///
/// ## Constraints
///
/// - `1 <= n <= 2^31 - 1`
///
/// ## Follow-up
///
/// If this function is called many times, consider optimizing using:
///
/// - Dynamic programming / lookup table
/// - Bit manipulation techniques
/// - Built-in bit count operations
pub struct Solutions;

impl Solutions {
    /// Returns the number of set bits (`1`s) in the binary representation of `n`.
    ///
    /// Also called the **Hamming weight** or **population count** of `n`.
    ///
    /// # Arguments
    ///
    /// * `n` — positive integer in `[1, 2^31 - 1]` (problem constraints).
    ///
    /// # Returns
    ///
    /// Count of bits equal to `1` in the two's-complement representation of `n`.
    ///
    /// # Algorithm
    ///
    /// Delegates to [`i32::count_ones`](i32::count_ones), which uses the target CPU's
    /// hardware population-count instruction when available (e.g. `POPCNT` on x86).
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: n = 11
    /// Output: 3
    /// Explanation: 11 = 0b1011 → three set bits
    /// ```
    ///
    /// ```text
    /// Input: n = 128
    /// Output: 1
    /// Explanation: 128 = 0b10000000 → one set bit
    /// ```
    ///
    /// ```text
    /// Input: n = 2147483645
    /// Output: 30
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(1)** for a fixed 32-bit word (constant number of bits).
    /// - Extra space: **O(1)**.
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}

#[cfg(test)]
mod number_of_1_bits_tests {
    use super::*;

    macro_rules! test_case {
        ($n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let n: i32 = $n;
                let expected: i32 = $expected;

                let result: i32 = Solutions::hamming_weight(n);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(11, 3, test_case_1);
    test_case!(128, 1, test_case_2);
    test_case!(2147483645, 30, test_case_3);
    test_case!(1, 1, test_case_4);
    test_case!(3, 2, test_case_5);
    test_case!(127, 7, test_case_6);
    test_case!(i32::MAX, 31, test_case_7);
    test_case!(2147483641, 29, test_case_8);
}
