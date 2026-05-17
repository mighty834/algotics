/// # Roman to Integer
///
/// Roman numerals are represented by seven different symbols:
///
/// ```text
/// Symbol    Value
/// I          1
/// V          5
/// X          10
/// L          50
/// C          100
/// D          500
/// M          1000
/// ```
///
/// For example:
///
/// - `2` is written as `II`
/// - `12` is written as `XII` (`X + II`)
/// - `27` is written as `XXVII` (`XX + V + II`)
///
/// Roman numerals are usually written from largest to smallest from left to right.
/// However, there are special subtraction cases:
///
/// - `I` can be placed before `V` and `X` to make `4` and `9`.
/// - `X` can be placed before `L` and `C` to make `40` and `90`.
/// - `C` can be placed before `D` and `M` to make `400` and `900`.
///
/// Given a Roman numeral, convert it to an integer.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: s = "III"
/// Output: 3
/// Explanation: III = 3
/// ```
///
/// ### Example 2:
/// ```text
/// Input: s = "LVIII"
/// Output: 58
/// Explanation:
/// L = 50
/// V = 5
/// III = 3
/// Total = 58
/// ```
///
/// ### Example 3:
/// ```text
/// Input: s = "MCMXCIV"
/// Output: 1994
/// Explanation:
/// M = 1000
/// CM = 900
/// XC = 90
/// IV = 4
/// Total = 1994
/// ```
///
/// ## Constraints
///
/// - `1 <= s.length <= 15`
/// - `s` contains only:
///   `('I', 'V', 'X', 'L', 'C', 'D', 'M')`
/// - `s` is guaranteed to be a valid Roman numeral
/// - `1 <= result <= 3999`
pub struct Solutions;

impl Solutions {
    /// Converts a valid Roman numeral string to its integer value.
    ///
    /// # Arguments
    ///
    /// * `s` — Roman numeral consisting only of `I`, `V`, `X`, `L`, `C`, `D`, `M`.
    ///   Guaranteed valid per the problem statement.
    ///
    /// # Returns
    ///
    /// The integer equivalent in `[1, 3999]`.
    ///
    /// # Algorithm
    ///
    /// 1. Collect `s` into a `Vec<char>` and map each symbol to its value via a `HashMap`.
    /// 2. Scan left to right. For `I`, `X`, or `C` that is not the last character, compare the
    ///    current value with the next:
    ///    - If the next is larger, add `right - left` (subtractive pair: IV, IX, XL, XC, CD, CM)
    ///      and skip the next index.
    ///    - Otherwise add the current symbol's value.
    /// 3. For all other symbols, add the current value directly.
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: s = "III"
    /// Output: 3
    /// ```
    ///
    /// ```text
    /// Input: s = "LVIII"
    /// Output: 58
    /// ```
    ///
    /// ```text
    /// Input: s = "MCMXCIV"
    /// Output: 1994
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(n)** where `n = s.len()`.
    /// - Extra space: **O(n)** for the char vector plus **O(1)** for the fixed-size symbol map.
    pub fn roman_to_int(s: String) -> i32 {
        let values = std::collections::HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let v: Vec<char> = s.chars().collect();
        let mut result = 0;

        let mut cx = 0;
        while cx < v.len() {
            if ['I', 'X', 'C'].contains(&v[cx]) && cx != v.len() - 1 {
                let left = values.get(&v[cx]).unwrap();
                let right = values.get(&v[cx + 1]).unwrap();

                if left < right {
                    result += right - left;
                    cx += 1;
                } else {
                    result += left;
                }
            } else {
                result += values.get(&v[cx]).unwrap();
            }

            cx += 1;
        }

        result
    }
}

#[cfg(test)]
mod roman_to_integer_tests {
    use super::*;

    macro_rules! test_case {
        ($s: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let s: String = String::from($s);
                let expected: i32 = $expected;

                let result: i32 = Solutions::roman_to_int(s);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!("III", 3, test_case_1);
    test_case!("LVIII", 58, test_case_2);
    test_case!("MCMXCIV", 1994, test_case_3);
    test_case!("I", 1, test_case_4);
    test_case!("V", 5, test_case_5);
    test_case!("D", 500, test_case_6);
    test_case!("M", 1000, test_case_7);
    test_case!("MMMCMXCIX", 3999, test_case_8);
}
