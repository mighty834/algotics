/// Returns the index of the first occurrence of a substring within a string.
///
/// Given two strings `haystack` and `needle`, returns the index of the first
/// occurrence of `needle` in `haystack`. If `needle` is not found, returns `-1`.
///
/// # Examples
///
/// ```text
/// Input: haystack = "sadbutsad", needle = "sad"
/// Output: 0
/// Explanation: "sad" occurs at indices 0 and 6.
/// The first occurrence is at index 0.
/// ```
///
/// ```text
/// Input: haystack = "leetcode", needle = "leeto"
/// Output: -1
/// Explanation: "leeto" does not occur in "leetcode".
/// ```
///
/// # Constraints
///
/// - `1 <= haystack.len(), needle.len() <= 10^4`
/// - `haystack` and `needle` consist of lowercase English letters (`'a'..='z'`)
///
/// # Requirements
///
/// - Return the index of the first occurrence of `needle` in `haystack`
/// - Return `-1` if `needle` is not found
///
/// # Notes
///
/// - The index is zero-based
/// - If multiple occurrences exist, return the first one
pub struct Solutions;

impl Solutions {
    /// Returns the zero-based index of the first occurrence of `needle` in `haystack`.
    ///
    /// **Idea**
    /// Delegates to [`str::find`](str::find): search `haystack` for the substring `needle`.
    /// If found, return that start index as `i32`; if not found, return `-1`.
    ///
    /// For this problem’s inputs (lowercase ASCII letters), the index returned by `find`
    /// matches a per-character index; in general `find` reports a **byte offset** into the
    /// UTF-8 string.
    ///
    /// # Arguments
    /// - `haystack`: String to search in.
    /// - `needle`: Substring to look for.
    ///
    /// # Returns
    /// The index of the first occurrence of `needle` in `haystack`, or `-1` if it does not
    /// occur.
    ///
    /// # Examples
    /// ```text
    /// haystack = "sadbutsad", needle = "sad"  -> 0
    /// haystack = "leetcode", needle = "leeto" -> -1
    /// ```
    ///
    /// # Complexity
    /// - Time: linear in `haystack.len()` for the substring search used by the standard
    ///   library (`find` on `str`).
    /// - Extra space: `O(1)` beyond the input strings.
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(index) => index as i32,
            None => -1
        }
    }
}

#[cfg(test)]
mod implement_str_str_tests {
    use super::*;

    macro_rules! test_case {
        ($haystack: expr, $needle: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let haystack: String = $haystack;
                let needle: String = $needle;
                let expected: i32 = $expected;

                let result: i32 = Solutions::str_str(haystack, needle);
                assert_eq!(result, expected);
            }
        }
    }

    test_case!(String::from("helloworld!"), String::from("world"), 5, test_case_1);
    test_case!(String::from("bigbangtheory"), String::from("omp"), -1, test_case_2);
    test_case!(String::from("a"), String::from("a"), 0, test_case_3);
    test_case!(String::from("highvoltagerocknroll"), String::from("rock"), 11, test_case_4);
    test_case!(String::from("bbb"), String::from("bbbb"), -1, test_case_5);
    test_case!(String::from("fnstrtostr"), String::from("str"), 2, test_case_6);
}