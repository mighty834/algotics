/// Finds the longest common prefix among a list of strings.
///
/// Given a vector of strings `strs`, returns the longest common prefix
/// shared by all strings. If there is no common prefix, returns an empty
/// string `""`.
///
/// # Examples
///
/// ```text
/// Input: strs = ["flower","flow","flight"]
/// Output: "fl"
/// ```
///
/// ```text
/// Input: strs = ["dog","racecar","car"]
/// Output: ""
/// Explanation: There is no common prefix among the input strings.
/// ```
///
/// # Constraints
///
/// - `1 <= strs.len() <= 200`
/// - `0 <= strs[i].len() <= 200`
/// - `strs[i]` consists of lowercase English letters (`'a'..='z'`) if non-empty
///
/// # Requirements
///
/// - Return the longest common prefix shared by all strings
/// - If no common prefix exists, return an empty string
///
/// # Notes
///
/// - The result may be an empty string
/// - Comparison is case-sensitive (only lowercase letters are used)
pub struct Solutions;

impl Solutions {
    /// Returns the longest common prefix shared by every string in `strs`.
    ///
    /// **Idea (column-wise scan)**
    /// - Let `max_len` be the length of the **shortest** string (no prefix can extend past it).
    /// - For each index `i` in `0..max_len`, take the character at `i` from `strs[0]`.
    /// - If **every** other string has the same character at `i`, append it to the result.
    /// - As soon as some string differs at `i`, stop: the common prefix ends before that column.
    ///
    /// # Arguments
    /// - `strs`: Non-empty vector of strings (see module-level constraints).
    ///
    /// # Returns
    /// The longest prefix common to all entries, or `""` if the first character already
    /// disagrees (or the shortest string is empty).
    ///
    /// # Examples
    /// ```text
    /// Input: strs = ["flower", "flow", "flight"]
    /// Output: "fl"
    ///
    /// Input: strs = ["dog", "racecar", "car"]
    /// Output: ""
    /// ```
    ///
    /// # Complexity
    /// - Time: grows with `strs.len()` and the minimum length; this implementation uses
    ///   `chars().nth(i)` for each string at each `i`, which rescans from the start of the
    ///   string each time (fine for the small bounds in the problem statement).
    /// - Extra space: `O(k)` for the returned prefix of length `k` (at most the shortest input).
    ///
    /// # Panics
    /// Panics if `strs` is empty (`min()` on an empty iterator), which violates the stated
    /// constraints.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let max_len: usize = strs.iter().map(|s| s.len()).min().unwrap();

        let mut result: String = String::new();
        'outer: for i in 0..max_len {

            let temp: char = strs[0].chars().nth(i).unwrap();
            for str in &strs {
                if str.chars().nth(i).unwrap() != temp {
                    break 'outer;
                }
            }

            result.push(temp);
        }

        result
    }
}

#[cfg(test)]
mod longest_common_prefix_tests {
    use super::*;

    macro_rules! test_case {
        ($strs: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let strs: Vec<String> = $strs;
                let expected: String = $expected;

                let result: String = Solutions::longest_common_prefix(strs);
                assert_eq!(result, expected);
            }
        }
    }

    test_case!(vec![
        String::from("hello"),
        String::from("hell"),
        String::from("he")
    ], String::from("he"), test_case_1);

    test_case!(vec![
        String::from("a"), String::from("a"), String::from("a")
    ], String::from("a"), test_case_2);

    test_case!(vec![
        String::from("high"),
        String::from("voltage"),
        String::from("end")
    ], String::from(""), test_case_3);

    test_case!(vec![
        String::from("knife"),
        String::from("knife"),
        String::from("no")
    ], String::from(""),test_case_4);

    test_case!(vec![
        String::from("gogogogo"),
        String::from("gogogo"),
        String::from("gogo")
    ], String::from("gogo"), test_case_5);

    test_case!(vec![
        String::from("noisybusy"),
        String::from("noisy"),
        String::from("noisy")
    ], String::from("noisy"), test_case_6);
}