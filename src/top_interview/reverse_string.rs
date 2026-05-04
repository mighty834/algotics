/// Reverses a string in-place.
///
/// The input string is provided as a mutable array of characters `s`.
/// The function must reverse the order of the characters directly
/// within the same array, without allocating additional memory.
///
/// # Examples
///
/// ```text
/// Input: s = ["h","e","l","l","o"]
/// Output: ["o","l","l","e","h"]
/// ```
///
/// ```text
/// Input: s = ["H","a","n","n","a","h"]
/// Output: ["h","a","n","n","a","H"]
/// ```
///
/// # Constraints
///
/// - `1 <= s.len() <= 10^5`
/// - `s[i]` is a printable ASCII character
///
/// # Requirements
///
/// - Modify the input array in-place
/// - Use `O(1)` extra memory
///
/// # Notes
///
/// - The input is a sequence of characters, not a UTF-8 string slice
/// - Only swapping elements is required; no reallocation should occur
pub struct Solutions;

impl Solutions {
    /// Reverse `s` in place using `Vec::reverse`.
    ///
    /// **Idea**
    /// Delegates to the standard library, which reorders elements within the same allocation.
    ///
    /// **Complexity**
    /// - Time: **O(n)** for `n = s.len()`.
    /// - Extra space: **O(1)** beyond the vector’s own buffer (matches typical in-place reversal).
    ///
    /// Prefer this when you want the canonical, allocation-free reversal.
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }

    /// Reverse by cloning, iterating reversed, collecting, then copying back.
    ///
    /// **Idea**
    /// - `clone` the buffer to `s_twin`.
    /// - Build a new `Vec<char>` with `s_twin.iter().rev().map(|c| *c).collect()`.
    /// - Copy into `s` with `clone_from_slice`.
    ///
    /// **Trade-offs**
    /// - Time: **O(n)**.
    /// - **Allocates** an auxiliary vector (the collected reverse) and holds a full clone of `s`;
    ///   this is **not** `O(1)` extra memory. Use for learning or when you explicitly want an
    ///   iterator pipeline; for interviews and large inputs, prefer [`reverse_string`](Self::reverse_string).
    pub fn reverse_string_with_iter(s: &mut Vec<char>) {
        let s_twin = s.clone();

        s.clone_from_slice(&s_twin.iter().rev().copied().collect::<Vec<char>>())
    }
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;

    macro_rules! test_case {
        ($s: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let mut s: Vec<char> = $s;
                let expected: Vec<char> = $expected;

                Solutions::reverse_string(&mut s);
                assert_eq!(s, expected);

                s = $s;
                Solutions::reverse_string_with_iter(&mut s);
                assert_eq!(s, expected);
            }
        };
    }

    test_case!(vec!['h', 'e', 'l', 'l', 'o'], vec!['o', 'l', 'l', 'e', 'h'], test_case_1);
    test_case!(vec!['A'], vec!['A'], test_case_2);
    test_case!(vec!['B', 'I', 'G'], vec!['G', 'I', 'B'], test_case_3);
    test_case!(vec!['m', 'a', 'd', 'a', 'm'], vec!['m', 'a', 'd', 'a', 'm'], test_case_4);
}
