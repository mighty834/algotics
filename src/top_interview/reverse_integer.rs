/// Reverses the digits of a signed 32-bit integer.
///
/// Given an integer `x`, returns the value obtained by reversing its digits.
/// If the reversed value falls outside the 32-bit signed integer range
/// `[-2^31, 2^31 - 1]`, the function returns `0`.
///
/// The solution must not use 64-bit integer types.
///
/// # Examples
///
/// ```text
/// Input: x = 123
/// Output: 321
/// ```
///
/// ```text
/// Input: x = -123
/// Output: -321
/// ```
///
/// ```text
/// Input: x = 120
/// Output: 21
/// ```
///
/// # Constraints
///
/// - `-2^31 <= x <= 2^31 - 1`
///
/// # Requirements
///
/// - Do not use 64-bit integers (`i64`, `u64`)
/// - Detect and handle overflow during reversal
///
/// # Notes
///
/// - The sign of the number should be preserved
/// - Leading zeros in the reversed result should be removed automatically
///   (e.g., `120 -> 21`)
pub struct Solutions;

impl Solutions {
    /// Reverse decimal digits of `x` with checked 32-bit arithmetic.
    ///
    /// # Algorithm used here
    ///
    /// 1. Repeatedly take `value % 10` and push into `digits`.
    ///    - For negative values, Rust `%` keeps the sign, so digits are negative
    ///      (e.g. `-123 -> [-3, -2, -1]`), which still reconstructs the correct result.
    /// 2. Rebuild the number by summing `digit * 10^position` from most significant place.
    /// 3. Use `checked_pow`, `checked_mul`, and `checked_add` in a `try_fold`.
    ///    - If any step overflows `i32`, folding returns `None` and the function returns `0`.
    ///
    /// This matches the problem rule: return `0` on overflow.
    ///
    /// # Complexity
    ///
    /// - Time: **O(d)** where `d` is the number of decimal digits.
    /// - Extra space: **O(d)** for the `digits` buffer.
    ///
    /// # Edge cases
    ///
    /// - `x == 0` returns `0` (empty `digits` fold starts from `0`).
    /// - Trailing zeros are naturally removed after reversal (`120 -> 21`, `-1000 -> -1`).
    pub fn reverse(x: i32) -> i32 {
        let mut digits: Vec<i32> = Vec::new();
        let mut value = x;
        while value != 0 {
            digits.push(value % 10);
            value /= 10;
        }

        digits
            .iter()
            .enumerate()
            .map(|(i, v)| (i + 1, v))
            .try_fold(0i32, |acc, (i, v)| {
                let summand = i32::checked_pow(10, (digits.len() - i) as u32)?;
                let mul = v.checked_mul(summand)?;
                acc.checked_add(mul)
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod reverse_integer_tests {
    use super::*;

    macro_rules! test_case {
        ($x: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let x: i32 = $x;
                let expected: i32 = $expected;

                let result: i32 = Solutions::reverse(x);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(123, 321, test_case_1);
    test_case!(1, 1, test_case_2);
    test_case!(-1000, -1, test_case_3);
    test_case!(56734, 43765, test_case_4);
    test_case!(i32::MAX, 0, test_case_5);
    test_case!(i32::MIN, 0, test_case_6);
    test_case!(15250, 5251, test_case_7);
}
