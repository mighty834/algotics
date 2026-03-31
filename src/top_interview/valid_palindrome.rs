/// Determines whether a string is a palindrome after normalization.
///
/// A phrase is considered a palindrome if, after:
/// - converting all uppercase letters to lowercase, and
/// - removing all non-alphanumeric characters,
/// it reads the same forward and backward.
///
/// Alphanumeric characters include letters (`a-z`, `A-Z`) and digits (`0-9`).
///
/// # Examples
///
/// ```text
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.
/// ```
///
/// ```text
/// Input: s = "race a car"
/// Output: false
/// Explanation: "raceacar" is not a palindrome.
/// ```
///
/// ```text
/// Input: s = " "
/// Output: true
/// Explanation: After removing non-alphanumeric characters, the string becomes empty,
/// which is considered a palindrome.
/// ```
///
/// # Constraints
///
/// - `1 <= s.len() <= 2 * 10^5`
/// - `s` consists of printable ASCII characters
///
/// # Requirements
///
/// - Ignore case differences
/// - Ignore non-alphanumeric characters
/// - Return `true` if the normalized string is a palindrome, otherwise `false`
///
/// # Notes
///
/// - An empty string is considered a palindrome
/// - Comparison should be case-insensitive
pub struct Solutions;

impl Solutions {
    /// Returns whether `s` is a palindrome after normalizing it.
    ///
    /// **Idea**
    /// - Keep only alphanumeric characters (`char::is_alphanumeric`).
    /// - Fold letters to lowercase (`to_lowercase`, which handles Unicode correctly for
    ///   multi-codepoint letters, though this problem is typically ASCII).
    /// - Compare the normalized sequence to its reverse: if equal, the string is a
    ///   palindrome under the problem rules.
    ///
    /// # Arguments
    /// - `s`: Input string (may contain spaces, punctuation, mixed case, etc.).
    ///
    /// # Returns
    /// `true` if the normalized string reads the same forward and backward; `false`
    /// otherwise. An empty normalized string (e.g. only spaces/punctuation) is treated as
    /// a palindrome.
    ///
    /// # Examples
    /// ```text
    /// Input: s = "A man, a plan, a canal: Panama"
    /// Output: true
    ///
    /// Input: s = "race a car"
    /// Output: false
    /// ```
    ///
    /// # Complexity
    /// - Time: `O(n)` over the length of `s` for filtering and building two collected strings
    /// - Extra space: `O(k)` where `k` is the length of the normalized string
    pub fn is_palindrome(s: String) -> bool {
        let s = s.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase());

        s.clone().collect::<String>() == s.rev().collect::<String>()
    }
}

#[cfg(test)]
mod valid_palindrome_tests {
    use super::*;

    macro_rules! test_case {
        ($s: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let s: String = $s;
                let expected: bool = $expected;

                let result: bool = Solutions::is_palindrome(s);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(String::from("Hello"), false, test_case_1);
    test_case!(String::from("Helloolleh"), true, test_case_2);
    test_case!(String::from("Go ! Go ! bar rabog    og"), true, test_case_3);
    test_case!(String::from("1_2-3%4%%5/4]3,,2.1"), true, test_case_4);
    test_case!(String::from("YOU"), false, test_case_5);
    test_case!(String::from(" "), true, test_case_6);
    test_case!(String::from("A"), true, test_case_7);
    test_case!(String::from("high % voltage | big ! bang..."), false, test_case_8);
    test_case!(String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"), false, test_case_9);
}
