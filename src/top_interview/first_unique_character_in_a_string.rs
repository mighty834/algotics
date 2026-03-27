/// Returns the index of the first non-repeating character in a string.
///
/// Given a string `s`, finds the first character that does not appear more than once
/// in the string and returns its index. If no such character exists, returns `-1`.
///
/// # Examples
///
/// ```text
/// Input: s = "leetcode"
/// Output: 0
/// Explanation: The character 'l' at index 0 does not repeat.
/// ```
///
/// ```text
/// Input: s = "loveleetcode"
/// Output: 2
/// ```
///
/// ```text
/// Input: s = "aabb"
/// Output: -1
/// ```
///
/// # Constraints
///
/// - `1 <= s.len() <= 10^5`
/// - `s` consists of lowercase English letters (`'a'..='z'`)
///
/// # Requirements
///
/// - Return the index of the first unique (non-repeating) character
/// - If no such character exists, return `-1`
///
/// # Notes
///
/// - The index is zero-based
/// - Only characters that appear exactly once are considered valid
pub struct Solutions;

impl Solutions {
    /// Returns the index of the first non-repeating character in `s`.
    ///
    /// **Idea**
    /// - Scan `s` once from left to right.
    /// - For each character, track:
    ///   - its **first index**
    ///   - its **occurrence count**
    /// - Among the characters that occur exactly once, return the smallest recorded index.
    ///
    /// **Data structure**
    /// Uses `HashMap<char, [i32; 2]>` where `[0] = first_index` and `[1] = count`.
    ///
    /// # Arguments
    /// - `s`: Input string.
    ///
    /// # Returns
    /// The zero-based index of the first character that appears exactly once in `s`,
    /// or `-1` if no such character exists.
    ///
    /// # Examples
    /// ```text
    /// Input: s = "leetcode"
    /// Output: 0
    ///
    /// Input: s = "loveleetcode"
    /// Output: 2
    ///
    /// Input: s = "aabb"
    /// Output: -1
    /// ```
    ///
    /// # Complexity
    /// - Time: `O(n)` average, where `n = s.chars().count()`
    /// - Extra space: `O(k)` where `k` is the number of distinct characters in `s`
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashMap;
        let mut storage: HashMap<char, [i32; 2]> = HashMap::with_capacity(s.len());

        for (i, c) in s.chars().enumerate() {
            if !storage.contains_key(&c) {
                storage.insert(c, [i as i32, 1]);
            } else {
                storage.get_mut(&c).unwrap()[1] += 1;
            }
        }

        storage
            .iter()
            .filter(|(c, [i, v])| *v == 1)
            .min_by_key(|tup| tup.1[0])
            .unwrap_or((&'c', &[-1, -1])).1[0]
    }

    /// Returns the index of the first non-repeating character in `s`.
    ///
    /// **Idea**
    /// Keep:
    /// - `good_values`: characters seen exactly once (mapped to their first index)
    /// - `bad_values`: characters seen more than once
    ///
    /// As you scan left-to-right:
    /// - if a character is already in `bad_values`, ignore it
    /// - else if it exists in `good_values`, remove it from `good_values` and add it to `bad_values`
    /// - otherwise, insert it into `good_values` with index `i`
    ///
    /// After the scan, the unique characters are exactly the keys left in `good_values`.
    /// Return the minimum stored index (or `-1` if there are none).
    ///
    /// # Arguments
    /// - `s`: Input string.
    ///
    /// # Returns
    /// The zero-based index of the first character that appears exactly once in `s`,
    /// or `-1` if no such character exists.
    ///
    /// # Examples
    /// ```text
    /// Input: s = "leetcode"
    /// Output: 0
    ///
    /// Input: s = "loveleetcode"
    /// Output: 2
    ///
    /// Input: s = "aabb"
    /// Output: -1
    /// ```
    ///
    /// # Complexity
    /// - Time: `O(n)` average, where `n = s.chars().count()`
    /// - Extra space: `O(k)` where `k` is the number of distinct characters in `s`
    pub fn first_uniq_char_by_mapset(s: String) -> i32 {
        use std::collections::{HashSet, HashMap};
        let mut good_values: HashMap<char, i32> = HashMap::with_capacity(s.len());
        let mut bad_values: HashSet<char> = HashSet::with_capacity(s.len());

        for (i, c) in s.chars().enumerate() {
            if !bad_values.contains(&c) {
                if good_values.contains_key(&c) {
                    good_values.remove(&c);
                    bad_values.insert(c);
                } else {
                    good_values.insert(c, i as i32);
                }
            }
        }

        *good_values
            .iter()
            .min_by_key(|(_, i)| *i)
            .unwrap_or((&'_', &-1)).1
    }
}

#[cfg(test)]
mod first_uniq_char_tests {
    use super::*;

    macro_rules! test_case {
        ($s: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let s: String = $s;
                let expected: i32 = $expected;

                let result: i32 = Solutions::first_uniq_char(s);
                assert_eq!(result, expected);

                let s: String = $s;
                let result: i32 = Solutions::first_uniq_char_by_mapset(s);
                assert_eq!(result, expected);
            }
        }
    }

    test_case!(String::from("hello"), 0, test_case_1);
    test_case!(String::from("abba"), -1, test_case_2);
    test_case!(String::from("abbadabba"), 4, test_case_3);
    test_case!(String::from("bananaboom"), 9, test_case_4);
    test_case!(String::from("qwertyytrewqasdfghhgfdsazxcvbnnbvcxz"), -1, test_case_5);
    test_case!(String::from("a"), 0, test_case_6);
    test_case!(String::from("qwertyytrewqasdflghhgfdsazxcvbnnbvcxz"), 16, test_case_7);
    test_case!(String::from("qwertyytrewqasdflghhgfdsazxcvbnnbvcxzp"), 16, test_case_8);
}