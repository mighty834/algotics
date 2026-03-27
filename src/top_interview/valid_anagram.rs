/// Determines whether one string is an anagram of another.
///
/// Given two strings `s` and `t`, returns `true` if `t` is an anagram of `s`,
/// and `false` otherwise.
///
/// An anagram is a word or phrase formed by rearranging the letters of another,
/// using all original characters exactly once.
///
/// # Examples
///
/// ```text
/// Input: s = "anagram", t = "nagaram"
/// Output: true
/// ```
///
/// ```text
/// Input: s = "rat", t = "car"
/// Output: false
/// ```
///
/// # Constraints
///
/// - `1 <= s.len(), t.len() <= 5 * 10^4`
/// - `s` and `t` consist of lowercase English letters (`'a'..='z'`)
///
/// # Requirements
///
/// - Return `true` if `t` is a valid anagram of `s`
/// - Otherwise, return `false`
///
/// # Follow-up
///
/// - How would the solution change if the input strings contain Unicode characters?
///
/// # Notes
///
/// - All characters must match in both frequency and identity
/// - Order of characters does not matter
pub struct Solutions;

impl Solutions {
    /// Checks whether `t` is an anagram of `s`.
    ///
    /// **Idea (sort and compare)**
    /// - If the lengths differ, they cannot be anagrams.
    /// - Convert both strings into `Vec<char>`, sort each vector, then compare them.
    /// - If sorted vectors are equal, both strings contain exactly the same characters
    ///   with the same frequencies, so the strings are anagrams.
    ///
    /// # Arguments
    /// - `s`: First input string.
    /// - `t`: Second input string.
    ///
    /// # Returns
    /// - `true` if `t` is an anagram of `s`
    /// - `false` otherwise
    ///
    /// # Examples
    /// ```text
    /// Input: s = "anagram", t = "nagaram"
    /// Output: true
    ///
    /// Input: s = "rat", t = "car"
    /// Output: false
    /// ```
    ///
    /// # Complexity
    /// - Time: `O(n log n)` from sorting, where `n = s.len() = t.len()`
    /// - Extra space: `O(n)` for the character vectors
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_vec: Vec<char> = s.chars().collect();
        let mut t_vec: Vec<char> = t.chars().collect();
        s_vec.sort();
        t_vec.sort();

        s_vec == t_vec
    }
}

#[cfg(test)]
mod valid_anagram_tests {
    use super::*;

    macro_rules! test_case {
        ($s: expr, $t: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let s: String = $s;
                let t: String = $t;
                let expected: bool = $expected;

                let result: bool = Solutions::is_anagram(s, t);
                assert_eq!(result, expected);
            }
        }
    }

    test_case!(String::from("hello"), String::from("olehl"), true, test_case_1);
    test_case!(String::from("gogogo"), String::from("ogogog"), true, test_case_2);
    test_case!(String::from("knife"), String::from("gnife"), false, test_case_3);
    test_case!(String::from("bang"), String::from("ganb"), true, test_case_4);
    test_case!(String::from("a"), String::from("a"), true, test_case_5);
    test_case!(String::from("night"), String::from("bight"), false, test_case_6);
    test_case!(String::from("bbbb"), String::from("bb"), false, test_case_7);
    test_case!(String::from("google"), String::from("googlegoogle"), false, test_case_8);
    test_case!(String::from("biggest"), String::from("gibbest"), false, test_case_9);
}