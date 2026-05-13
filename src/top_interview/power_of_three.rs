/// # Power of Three
///
/// Given an integer `n`, return `true` if it is a power of three. Otherwise, return `false`.
///
/// An integer `n` is a power of three if there exists an integer `x` such that:
/// `n == 3^x`.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: n = 27
/// Output: true
/// Explanation: 27 = 3^3
/// ```
///
/// ### Example 2:
/// ```text
/// Input: n = 0
/// Output: false
/// Explanation: There is no integer x such that 3^x = 0.
/// ```
///
/// ### Example 3:
/// ```text
/// Input: n = -1
/// Output: false
/// Explanation: There is no integer x such that 3^x = -1.
/// ```
///
/// ## Constraints
///
/// - `-2^31 <= n <= 2^31 - 1`
pub struct Solutions;

impl Solutions {
    pub fn is_power_of_three(n: i32) -> bool {
        if n == 1 {
            return true;
        }

        let mut pow = 3;
        while pow < n {
            if let Some(next) = pow.checked_mul(3) {
                pow = next;
            } else {
                break;
            }
        }

        pow == n
    }
}

#[cfg(test)]
mod power_of_three_tests {
    use super::*;

    macro_rules! test_case {
        ($n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let n: i32 = $n;
                let expected: bool = $expected;
                let result = Solutions::is_power_of_three(n);

                assert_eq!(result, expected);
            }
        };
    }

    test_case!(27, true, test_case_1);
    test_case!(0, false, test_case_2);
    test_case!(-1, false, test_case_3);
    test_case!(100, false, test_case_4);
    test_case!(9, true, test_case_5);
    test_case!(i32::MAX, false, test_case_6);
    test_case!(i32::MIN, false, test_case_7);
    test_case!(81, true, test_case_8);
    test_case!(1, true, test_case_9);
    test_case!(-9, false, test_case_10);
    test_case!(-3, false, test_case_11);
    test_case!(-27, false, test_case_12);
}
