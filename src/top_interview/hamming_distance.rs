/// # Hamming Distance
///
/// The Hamming distance between two integers is the number of positions at which
/// the corresponding bits are different.
///
/// Given two integers `x` and `y`, return the Hamming distance between them.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: x = 1, y = 4
/// Output: 2
///
/// Explanation:
/// 1   (0 0 0 1)
/// 4   (0 1 0 0)
///        ↑   ↑
///
/// The arrows indicate positions where the corresponding bits differ.
/// ```
///
/// ### Example 2:
/// ```text
/// Input: x = 3, y = 1
/// Output: 1
/// ```
///
/// ## Constraints
///
/// - `0 <= x, y <= 2^31 - 1`
///
/// ## Notes
///
/// - This problem is equivalent to:
///   "Minimum Bit Flips to Convert Number".
/// - A common solution uses:
///   `x ^ y` (XOR), then counts the number of set bits.
///
/// ## Complexity
///
/// Typical bit manipulation solution:
///
/// - Time: `O(number_of_bits)`
/// - Space: `O(1)`
pub struct Solutions;

impl Solutions {
    /// Returns the Hamming distance between two integers.
    ///
    /// The Hamming distance is the number of bit positions where `x` and `y` differ
    /// (equivalently, the number of set bits in `x ^ y`).
    ///
    /// # Arguments
    ///
    /// * `x` — first integer in `[0, 2^31 - 1]`.
    /// * `y` — second integer in `[0, 2^31 - 1]`.
    ///
    /// # Returns
    ///
    /// Count of positions at which the binary representations of `x` and `y` differ.
    ///
    /// # Algorithm
    ///
    /// 1. Format `x` and `y` as binary strings (`format!("{:b}", …)`), reverse each so index
    ///    `0` is the least significant bit.
    /// 2. Compare the overlapping prefix (length `min(len(x), len(y))`); increment the count
    ///    when the two digits differ.
    /// 3. For any extra bits in the longer representation, count each `'1'` as one more
    ///    differing position (implicit leading zeros in the shorter number).
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: x = 1, y = 4
    /// Output: 2
    /// Explanation: 1 = 0b001, 4 = 0b100 → two differing bit positions
    /// ```
    ///
    /// ```text
    /// Input: x = 3, y = 1
    /// Output: 1
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(b)** where `b` is the number of bits in the longer representation (at most 31
    ///   for valid inputs).
    /// - Extra space: **O(b)** for the two reversed binary `Vec<char>` buffers.
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let x: Vec<char> = format!("{:b}", x).chars().rev().collect();
        let y: Vec<char> = format!("{:b}", y).chars().rev().collect();

        let min = x.len().min(y.len());
        let mut result = 0;
        for i in 0..min {
            if x[i] != y[i] {
                result += 1;
            }
        }

        let max = x.len().max(y.len());
        let m = if x.len() > y.len() { x } else { y };
        for i in min..max {
            if m[i] != '0' {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod hamming_distance_tests {
    use super::*;

    macro_rules! test_case {
        ($x: expr, $y: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let x: i32 = $x;
                let y: i32 = $y;
                let expected: i32 = $expected;

                let result: i32 = Solutions::hamming_distance(x, y);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(1, 4, 2, test_case_1);
    test_case!(3, 1, 1, test_case_2);
    test_case!(7, 1, 2, test_case_3);
    test_case!(15, 1, 3, test_case_4);
    test_case!(15, 7, 1, test_case_5);
    test_case!(16, 15, 5, test_case_6);
    test_case!(31, 17, 3, test_case_7);
    test_case!(i32::MAX, 0, 31, test_case_8);
}
