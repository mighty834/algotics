/// # Fizz Buzz
///
/// Given an integer `n`, return a string array `answer` (1-indexed) where:
///
/// - `answer[i] == "FizzBuzz"` if `i` is divisible by both `3` and `5`.
/// - `answer[i] == "Fizz"` if `i` is divisible by `3`.
/// - `answer[i] == "Buzz"` if `i` is divisible by `5`.
/// - `answer[i] == i` (as a string) if none of the above conditions are true.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: n = 3
/// Output: ["1","2","Fizz"]
/// ```
///
/// ### Example 2:
/// ```text
/// Input: n = 5
/// Output: ["1","2","Fizz","4","Buzz"]
/// ```
///
/// ### Example 3:
/// ```text
/// Input: n = 15
/// Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz",
///          "11","Fizz","13","14","FizzBuzz"]
/// ```
///
/// ## Constraints
///
/// - `1 <= n <= 10^4`
pub struct Solutions;

impl Solutions {
    /// Generates the classic **FizzBuzz** sequence from `1` to `n` (inclusive).
    ///
    /// For each integer `i` in `1..=n`:
    ///
    /// - if `i` is divisible by **3** and **5**, emit `"FizzBuzz"`
    /// - else if divisible by **3**, emit `"Fizz"`
    /// - else if divisible by **5**, emit `"Buzz"`
    /// - otherwise emit `i.to_string()`
    ///
    /// # Arguments
    ///
    /// * `n` — Upper bound (inclusive). Typical constraints are `1..=10^4`.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` of length `n` (for `n >= 1`) containing the FizzBuzz output.
    ///
    /// # Complexity
    ///
    /// - **Time:** `O(n)`
    /// - **Extra space:** `O(n)` for the output vector
    ///
    /// # Notes
    ///
    /// For `n <= 0`, the loop `1..=n` produces no items and this returns an empty vector. The
    /// intended problem input uses `n >= 1`.
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                result.push("FizzBuzz".to_string());
                continue;
            }

            if i % 3 == 0 {
                result.push("Fizz".to_string());
                continue;
            }

            if i % 5 == 0 {
                result.push("Buzz".to_string());
                continue;
            }

            result.push(i.to_string());
        }

        result
    }
}

#[cfg(test)]
mod fizz_buzz_tests {
    use super::*;

    macro_rules! test_case {
        ($n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let n: i32 = $n;
                let mut expected: Vec<String> = Vec::new();
                for s in $expected {
                    expected.push(String::from(s))
                }

                let result: Vec<String> = Solutions::fizz_buzz(n);

                assert_eq!(result, expected);
            }
        };
    }

    test_case!(3, vec!["1", "2", "Fizz"], test_case_1);
    test_case!(5, vec!["1", "2", "Fizz", "4", "Buzz"], test_case_2);
    test_case!(
        15,
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ],
        test_case_3
    );
    test_case!(
        21,
        vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "Fizz"
        ],
        test_case_4
    );
}
