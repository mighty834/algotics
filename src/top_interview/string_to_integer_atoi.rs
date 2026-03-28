/// Converts a string to a 32-bit signed integer (similar to `atoi`).
///
/// The function parses the input string `s` according to the following rules:
///
/// 1. **Whitespace**:
///    - Ignore any leading whitespace characters (`' '`).
///
/// 2. **Signedness**:
///    - Check the next character for a sign:
///      - `'-'` → result is negative
///      - `'+'` → result is positive
///      - otherwise → assume positive
///
/// 3. **Conversion**:
///    - Read consecutive digits (`'0'..='9'`) and construct the integer
///    - Skip leading zeros in the result
///    - Stop reading when a non-digit character is encountered or the end of the string is reached
///    - If no digits are read, return `0`
///
/// 4. **Clamping (Overflow Handling)**:
///    - If the parsed integer is outside the 32-bit signed integer range
///      `[-2^31, 2^31 - 1]`, clamp the result:
///      - values < `-2^31` → return `-2^31`
///      - values > `2^31 - 1` → return `2^31 - 1`
///
/// # Examples
///
/// ```text
/// Input: s = "42"
/// Output: 42
/// ```
///
/// ```text
/// Input: s = "   -042"
/// Output: -42
/// ```
///
/// ```text
/// Input: s = "1337c0d3"
/// Output: 1337
/// ```
///
/// ```text
/// Input: s = "0-1"
/// Output: 0
/// ```
///
/// ```text
/// Input: s = "words and 987"
/// Output: 0
/// ```
///
/// # Constraints
///
/// - `0 <= s.len() <= 200`
/// - `s` consists of:
///   - English letters (`a-z`, `A-Z`)
///   - digits (`0-9`)
///   - whitespace (`' '`)
///   - `'+'`, `'-'`, `'.'`
///
/// # Requirements
///
/// - Ignore leading whitespace
/// - Handle optional sign
/// - Parse digits until a non-digit is encountered
/// - Clamp result to 32-bit signed integer range
///
/// # Notes
///
/// - Parsing stops at the first non-digit character after optional sign and digits
/// - An empty or invalid numeric prefix results in `0`
/// - No use of 64-bit integers is required
pub struct Solutions;

impl Solutions {
    /// Parses `s` into a 32-bit signed integer using `atoi`-style rules.
    ///
    /// **Idea**
    /// - [`str::trim`](str::trim) the input, then handle an optional leading `'+'` or `'-'`.
    /// - Walk characters from the start of the remaining slice and keep the longest prefix
    ///   of ASCII digits (`'0'..='9'`); stop at the first non-digit.
    /// - Prepend the sign and call [`str::parse::<i32>`](str::parse) on that string.
    /// - On parse success, return the value. On parse failure:
    ///   - if the digit part is missing or invalid as a number, return `0`;
    ///   - if the digit string overflows `i32`, return [`i32::MAX`] or [`i32::MIN`] according
    ///     to the sign.
    ///
    /// # Arguments
    /// - `s`: String to parse (see module-level constraints for allowed characters).
    ///
    /// # Returns
    /// The parsed integer, `0` when no valid number is read, or a clamped value when the
    /// magnitude exceeds the `i32` range.
    ///
    /// # Examples
    /// ```text
    /// Input: s = "42"
    /// Output: 42
    ///
    /// Input: s = "   -042"
    /// Output: -42
    ///
    /// Input: s = "words and 987"
    /// Output: 0
    /// ```
    ///
    /// # Complexity
    /// - Time: `O(n)` for `n = s.len()` (trim, scan, parse attempt)
    /// - Extra space: `O(n)` for the temporary `String` built before parsing
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim();
        let is_positive: bool;

        if s.len() == 0 {
            return 0;
        }

        if s.len() == 1 {
            let c = s.chars().next().unwrap();
            if c == '+' || c == '-' {
                return 0;
            }
        }

        if s.starts_with('-') {
            is_positive = false;
            s = &s[1..];
        } else {
            is_positive = true;
            if s.starts_with('+') {
                s = &s[1..];
            }
        }

        let mut cx = 0;
        for (i, c) in s.chars().enumerate() {
            if c.is_numeric() {
                cx = i;
            } else {
                break;
            }
        }

        s = &s[0..=cx];

        let s = if is_positive {
            format!("+{}", s)
        } else {
            format!("-{}", s)
        };

        let result: i32 = match s.parse::<i32>() {
            Ok(x) => x as i32,
            Err(_) => {
                if !s.chars().skip(1).next().unwrap().is_numeric() {
                    return 0;
                }

                if is_positive {
                    i32::MAX
                } else {
                    i32::MIN
                }
            }
        };

        result
    }
}

mod string_to_integer_atoi {
    use super::*;

    macro_rules! test_case {
        ($s: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let s: String = $s;
                let expected: i32 = $expected;

                let result: i32 = Solutions::my_atoi(s);
                assert_eq!(result, expected);
            }
        }
    }

    test_case!(String::from(""), 0, test_case_1);
    test_case!(String::from("+10"), 10, test_case_2);
    test_case!(String::from("-10"), -10, test_case_3);
    test_case!(String::from("Hello world"), 0, test_case_4);
    test_case!(String::from("   100    "), 100, test_case_5);
    test_case!(String::from("Hello100"), 0, test_case_6);
    test_case!(String::from("     +90Hello"), 90, test_case_7);
    test_case!(String::from(" -199999World   "), -199999, test_case_8);
    test_case!(String::from("5000000000"), i32::MAX, test_case_9);
    test_case!(String::from("-5000000000"), i32::MIN, test_case_10);
    test_case!(String::from("12345 123"), 12345, test_case_11);
    test_case!(String::from("+"), 0, test_case_12);
    test_case!(String::from("20000000000000000000"), i32::MAX, test_case_13);
    test_case!(String::from("1234567890123456789012345678901234567890"), i32::MAX, test_case_14);
}