/// # Reverse Bits
///
/// Reverse the bits of a given 32-bit unsigned integer.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: n = 43261596
/// Output: 964176192
///
/// Explanation:
///
/// Integer     Binary
/// 43261596    00000010100101000001111010011100
/// 964176192   00111001011110000010100101000000
/// ```
///
/// ### Example 2:
/// ```text
/// Input: n = 2147483644
/// Output: 1073741822
///
/// Explanation:
///
/// Integer     Binary
/// 2147483644  01111111111111111111111111111100
/// 1073741822  00111111111111111111111111111110
/// ```
///
/// ## Constraints
///
/// - Input is treated as an unsigned 32-bit integer (all 32 bits are reversed).
/// - `0 <= n <= 2^31 - 1` for typical LeetCode inputs.
///
/// ## Follow-up
///
/// If this function is called many times, consider optimizing with:
///
/// - Bit manipulation techniques
/// - Lookup tables (precomputing reversed values for bytes)
///
/// ## Complexity
///
/// Standard bit-by-bit solution:
///
/// - Time: `O(32)`
/// - Space: `O(1)`
pub struct Solutions;

impl Solutions {
    /// Reverse all 32 bits of `n` using the standard library.
    ///
    /// Casts `n` to `u32` and calls [`u32::reverse_bits`], then casts back to `i32`.
    /// Bit `i` of the input becomes bit `31 - i` of the output (full 32-bit reversal,
    /// not variable-width).
    ///
    /// # Algorithm
    ///
    /// Delegates to the compiler / CPU bit-reversal on a fixed 32-bit word — no manual
    /// loops or string formatting.
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: n = 43261596
    /// Output: 964176192
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(1)** for a fixed 32-bit word.
    /// - Extra space: **O(1)**.
    ///
    /// # See also
    ///
    /// [`custom_reverse_bits`](Self::custom_reverse_bits),
    /// [`primitive_reverse_bits`](Self::primitive_reverse_bits).
    pub fn reverse_bits(n: i32) -> i32 {
        n.reverse_bits() as i32
    }

    /// Reverse all 32 bits of `n` by reversing a zero-padded binary string.
    ///
    /// # Algorithm
    ///
    /// 1. Format `n` as exactly 32 characters with `format!("{:032b}", n)`.
    /// 2. Reverse the character sequence and parse back with [`i32::from_str_radix`].
    ///
    /// A teaching alternative to [`reverse_bits`](Self::reverse_bits); allocates
    /// temporary strings but mirrors the problem statement literally.
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: n = 43261596
    /// Output: 964176192
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(32)**.
    /// - Extra space: **O(32)** for the formatted and reversed strings.
    ///
    /// # Panics
    ///
    /// Panics if binary parsing fails (should not happen for valid 32-bit patterns).
    ///
    /// # See also
    ///
    /// [`reverse_bits`](Self::reverse_bits),
    /// [`primitive_reverse_bits`](Self::primitive_reverse_bits).
    pub fn custom_reverse_bits(n: i32) -> i32 {
        let bits = format!("{:032b}", n);
        let rev_bits = bits.chars().rev().collect::<String>();

        i32::from_str_radix(&rev_bits, 2).unwrap()
    }

    /// Reverse all 32 bits of `n` with an explicit index loop (no `reverse_bits` intrinsics).
    ///
    /// # Algorithm
    ///
    /// 1. Format `n` in binary **without** leading-zero padding (`{:b}`).
    /// 2. For `i` in `1..=32`, take the `i`-th bit from the right of the unpadded
    ///    representation, or `'0'` if that position lies left of the most significant bit.
    /// 3. Parse the 32-character result as binary.
    ///
    /// Illustrates manual padding to a fixed 32-bit width without `{:032b}` or
    /// [`u32::reverse_bits`].
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: n = 43261596
    /// Output: 964176192
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(32)**.
    /// - Extra space: **O(32)** for the char buffer and result string.
    ///
    /// # Panics
    ///
    /// Panics if binary parsing fails (should not happen for valid 32-bit patterns).
    ///
    /// # See also
    ///
    /// [`reverse_bits`](Self::reverse_bits),
    /// [`custom_reverse_bits`](Self::custom_reverse_bits).
    pub fn primitive_reverse_bits(n: i32) -> i32 {
        let bits = format!("{:b}", n).chars().collect::<Vec<char>>();
        let mut rev_bits = String::new();

        for i in 1..=32 {
            let index = bits.len() as i32 - i;

            if index >= 0 {
                rev_bits.push(bits[index as usize]);
            } else {
                rev_bits.push('0');
            }
        }

        i32::from_str_radix(&rev_bits, 2).unwrap()
    }
}

#[cfg(test)]
mod reverse_bits_tests {
    use super::*;

    macro_rules! test_case {
        ($n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let n: i32 = $n;
                let expected: i32 = $expected;

                let result: i32 = Solutions::reverse_bits(n);
                assert_eq!(result, expected);

                let result: i32 = Solutions::custom_reverse_bits(n);
                assert_eq!(result, expected);

                let result: i32 = Solutions::primitive_reverse_bits(n);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(0, 0, test_case_1);
    test_case!(43261596, 964176192, test_case_2);
    test_case!(2147483644, 1073741822, test_case_3);

    test_case!(
        i32::from_str_radix("01000000100101000001001010010100", 2).unwrap(),
        i32::from_str_radix("00101001010010000010100100000010", 2).unwrap(),
        test_case_4
    );

    test_case!(
        i32::from_str_radix("01010100110101000001001010010110", 2).unwrap(),
        i32::from_str_radix("01101001010010000010101100101010", 2).unwrap(),
        test_case_5
    );
}
