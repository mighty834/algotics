/// # Valid Parentheses
///
/// Given a string `s` containing only the characters:
///
/// ```text
/// '(', ')', '{', '}', '[' and ']'
/// ```
///
/// Determine whether the input string is valid.
///
/// An input string is considered valid if:
///
/// - Open brackets must be closed by the same type of brackets.
/// - Open brackets must be closed in the correct order.
/// - Every closing bracket must have a corresponding opening bracket
///   of the same type.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: s = "()"
/// Output: true
/// ```
///
/// ### Example 2:
/// ```text
/// Input: s = "()[]{}"
/// Output: true
/// ```
///
/// ### Example 3:
/// ```text
/// Input: s = "(]"
/// Output: false
/// ```
///
/// ### Example 4:
/// ```text
/// Input: s = "([])"
/// Output: true
/// ```
///
/// ### Example 5:
/// ```text
/// Input: s = "([)]"
/// Output: false
/// ```
///
/// ## Constraints
///
/// - `1 <= s.length <= 10^4`
/// - `s` consists only of the characters:
///   `'('`, `')'`, `'{'`, `'}'`, `'['`, `']'`
///
/// ## Notes
///
/// - A common solution uses a stack:
///   - Push opening brackets onto the stack.
///   - For each closing bracket:
///     - Check whether the top of the stack contains the matching opening bracket.
///   - The string is valid if the stack is empty at the end.
///
/// ## Complexity
///
/// Stack-based solution:
///
/// - Time: `O(n)`
/// - Space: `O(n)`
pub struct Solutions;

impl Solutions {
    /// Returns whether `s` is a correctly nested and matched bracket string.
    ///
    /// The string may contain only `(`, `)`, `{`, `}`, `[`, and `]`. It is valid when every
    /// closing bracket matches the most recent unmatched opening bracket of the same kind and
    /// no brackets remain open at the end.
    ///
    /// # Arguments
    ///
    /// * `s` — Bracket-only input. Expected length `1..=10^4` per problem constraints.
    ///
    /// # Returns
    ///
    /// `true` if all brackets are properly paired and ordered; `false` otherwise.
    ///
    /// # Algorithm
    ///
    /// Classic **stack** scan:
    ///
    /// 1. For each character, if it is an opener (`(`, `{`, `[`), push it onto the stack.
    /// 2. If it is a closer, return `false` when the stack is empty.
    /// 3. Otherwise pop the top opener and return `false` unless it pairs with the closer
    ///    (`(` with `)`, `{` with `}`, `[` with `]`).
    /// 4. After the scan, return whether the stack is empty (no unmatched openers).
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: s = "()"
    /// Output: true
    /// ```
    ///
    /// ```text
    /// Input: s = "()[]{}"
    /// Output: true
    /// ```
    ///
    /// ```text
    /// Input: s = "(]"
    /// Output: false
    /// ```
    ///
    /// ```text
    /// Input: s = "([)]"
    /// Output: false
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(n)** where `n = s.len()` (one pass, constant work per character).
    /// - Extra space: **O(n)** for the stack in the worst case (all openers).
    ///
    /// # Panics
    ///
    /// Panics if `stack.pop()` is called on an empty stack after the empty-stack check
    /// (unreachable on the current control flow).
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            if ['(', '{', '['].contains(&c) {
                stack.push(c);
            } else {
                if stack.is_empty() {
                    return false;
                }

                let last = stack.pop().unwrap();
                if last == '(' && c != ')' || last == '{' && c != '}' || last == '[' && c != ']' {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod valid_parentheses_tests {
    use super::*;

    macro_rules! test_case {
        ($s: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let s: String = String::from($s);
                let expected: bool = $expected;

                let result: bool = Solutions::is_valid(s);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!("()", true, test_case_1);
    test_case!("()[]{}", true, test_case_2);
    test_case!("(]", false, test_case_3);
    test_case!("([])", true, test_case_4);
    test_case!("([)]", false, test_case_5);
    test_case!("(", false, test_case_6);
    test_case!("}", false, test_case_7);
    test_case!("({[[](){}]})", true, test_case_8);
    test_case!("(((((((((())))))))))[[[[[{{{{{}}}}}]]]]]", true, test_case_9);
    test_case!("(((((((((())))))))))[[[[[[{{{{{}}}}}]]]]]", false, test_case_10);
}
