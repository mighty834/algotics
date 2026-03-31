/// Increments a large integer represented as an array of digits.
///
/// The integer is given as a vector `digits`, where each element represents
/// a single digit in base 10. The digits are ordered from most significant
/// to least significant (left to right).
///
/// The input integer does not contain any leading zeros.
///
/// The function should increment the integer by one and return the resulting
/// array of digits.
///
/// # Examples
///
/// ```text
/// Input: digits = [1,2,3]
/// Output: [1,2,4]
/// Explanation: The array represents the integer 123.
/// Incrementing by one gives 124.
/// ```
///
/// ```text
/// Input: digits = [4,3,2,1]
/// Output: [4,3,2,2]
/// Explanation: The array represents the integer 4321.
/// Incrementing by one gives 4322.
/// ```
///
/// ```text
/// Input: digits = [9]
/// Output: [1,0]
/// Explanation: The array represents the integer 9.
/// Incrementing by one gives 10.
/// ```
///
/// # Constraints
///
/// - `1 <= digits.len() <= 100`
/// - `0 <= digits[i] <= 9`
/// - The input does not contain leading zeros
///
/// # Notes
///
/// - The digits are stored in big-endian order (most significant digit first).
/// - The result may increase the length of the array (e.g., `[9] -> [1, 0]`).
pub struct Solutions;

impl Solutions {
    /// Increment the big-endian digit array by `1` in-place on a mutable copy.
    ///
    /// **Idea**
    /// - Clone `digits` into `result` and scan **from the least significant digit**
    ///   (right to left).
    /// - If `result[i] != 9`, add 1 and return immediately — no carry beyond this position.
    /// - If `result[i] == 9`, set it to `0` and continue left; carry propagates.
    /// - If every digit was `9`, the loop ends with all zeros; insert `1` at the front
    ///   (e.g. `[9] -> [1, 0]`, `[9,9,9] -> [1, 0, 0, 0]`).
    ///
    /// This implements grade-school addition of `+1` without converting the whole number
    /// to an integer type (which would overflow for very long digit strings).
    ///
    /// # Complexity
    ///
    /// - Time: **O(n)** where `n = digits.len()` (one pass from the end; worst case all `9`s).
    /// - Extra space: **O(n)** for the cloned vector (plus `Vec::insert` may reallocate).
    ///
    /// # Panics
    ///
    /// Never panics for valid inputs per the constraints (`0..=9` per digit). Invalid digits
    /// are not validated.
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = digits.clone();

        for i in (0..result.len()).rev() {
            if result[i] != 9 {
                result[i] += 1;
                return result;
            }

            result[i] = 0;
        }

        result.insert(0, 1);
        result
    }
}

#[cfg(test)]
mod plus_one_tests {

    use super::*;

    macro_rules! test_case {
        ($digits: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let digits: Vec<i32> = $digits;
                let expected: Vec<i32> = $expected;

                let result = Solutions::plus_one(digits);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 6], test_case_1);
    test_case!(vec![1], vec![2], test_case_2);
    test_case!(vec![9], vec![1, 0], test_case_3);
    test_case!(vec![9, 9, 9, 9, 9], vec![1, 0, 0, 0, 0, 0], test_case_4);
    test_case!(vec![1, 8, 9], vec![1, 9, 0], test_case_5);
    test_case!(vec![1, 0], vec![1, 1], test_case_6);
    test_case!(vec![5, 5, 5], vec![5, 5, 6], test_case_7);
}
