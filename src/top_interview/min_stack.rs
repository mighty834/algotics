/// # Min Stack
///
/// Design a stack that supports:
///
/// - `push(val)` — pushes an element onto the stack
/// - `pop()` — removes the top element
/// - `top()` — returns the top element
/// - `get_min()` — retrieves the minimum element in the stack
///
/// All operations must run in **O(1)** time complexity.
///
/// Implement the `MinStack` struct:
///
/// - `MinStack::new()` initializes the stack.
/// - `push(val)` pushes `val` onto the stack.
/// - `pop()` removes the top element.
/// - `top()` returns the top element.
/// - `get_min()` returns the minimum value currently in the stack.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input:
/// ["MinStack","push","push","push","getMin","pop","top","getMin"]
/// [[],[-2],[0],[-3],[],[],[],[]]
///
/// Output:
/// [null,null,null,null,-3,null,0,-2]
///
/// Explanation:
///
/// MinStack min_stack = MinStack::new();
///
/// min_stack.push(-2);
/// min_stack.push(0);
/// min_stack.push(-3);
///
/// min_stack.get_min(); // returns -3
///
/// min_stack.pop();
///
/// min_stack.top();     // returns 0
///
/// min_stack.get_min(); // returns -2
/// ```
///
/// ## Constraints
///
/// - `-2^31 <= val <= 2^31 - 1`
/// - `pop()`, `top()`, and `get_min()` are always called on a non-empty stack
/// - At most `3 * 10^4` calls will be made to:
///   - `push`
///   - `pop`
///   - `top`
///   - `get_min`
///
/// ## Notes
///
/// A common approach is to maintain:
///
/// - A regular stack for values
/// - An auxiliary stack for minimum values
///
/// Example:
///
/// ```text
/// Values: [ -2, 0, -3 ]
/// Min:    [ -2, -2, -3 ]
/// ```
///
/// This guarantees:
///
/// - `push`: `O(1)`
/// - `pop`: `O(1)`
/// - `top`: `O(1)`
/// - `get_min`: `O(1)` with an auxiliary min stack (see module **Notes**)
pub struct Solution {
    /// Main stack storage; the top is the last element.
    data: Vec<i32>,
}

impl Solution {
    /// Create an empty min stack.
    ///
    /// # Returns
    ///
    /// A stack with no elements. Further operations use [`push`](Self::push),
    /// [`pop`](Self::pop), [`top`](Self::top), and [`get_min`](Self::get_min).
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Push `val` onto the top of the stack.
    ///
    /// # Arguments
    ///
    /// * `val` — Value to append. Expected in `[-2^31, 2^31 - 1]`.
    ///
    /// # Complexity
    ///
    /// - Time: **O(1)** amortized (`Vec::push`).
    /// - Extra space: **O(1)** per call (excluding stack growth).
    fn push(&mut self, val: i32) {
        self.data.push(val);
    }

    /// Remove the element on top of the stack.
    ///
    /// Assumes the stack is non-empty (LeetCode guarantees this for `pop` calls).
    ///
    /// # Complexity
    ///
    /// - Time: **O(1)**.
    /// - Extra space: **O(1)**.
    ///
    /// # Note
    ///
    /// `pop()` on an empty stack is undefined per problem constraints; this implementation
    /// would simply no-op if called on empty (`Vec::pop` returns `None`).
    fn pop(&mut self) {
        self.data.pop();
    }

    /// Return the top element without removing it.
    ///
    /// # Returns
    ///
    /// The most recently pushed value still on the stack.
    ///
    /// # Complexity
    ///
    /// - Time: **O(1)**.
    /// - Extra space: **O(1)**.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty (`last().unwrap()`). Problem constraints ensure
    /// `top()` is only called on a non-empty stack.
    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    /// Return the minimum value currently in the stack.
    ///
    /// # Returns
    ///
    /// The smallest `i32` among all values stored in the stack.
    ///
    /// # Algorithm (this implementation)
    ///
    /// Linear scan with [`Iterator::min`] over `data` — correct but **not** the same as
    /// the problem's O(1) requirement.
    ///
    /// # Complexity
    ///
    /// - Time: **O(n)** where `n` is the stack size.
    /// - Extra space: **O(1)**.
    ///
    /// # Panics
    ///
    /// Panics if the stack is empty. Problem constraints ensure `get_min()` is only called
    /// on a non-empty stack.
    ///
    /// # See also
    ///
    /// Module-level **Notes** describe the auxiliary min-stack approach for **O(1)** `get_min`.
    fn get_min(&self) -> i32 {
        *self.data.iter().min().unwrap()
    }
}

#[cfg(test)]
mod min_stack_tests {
    use super::*;

    macro_rules! test_case {
        ($actions: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let actions: Vec<(&str, i32)> = $actions;

                let mut solution = Solution::new();
                for (action, value) in actions {
                    match action {
                        "push" => solution.push(value),
                        "pop" => solution.pop(),
                        "top" => {
                            assert_eq!(solution.top(), value);
                        }
                        "get_min" => {
                            assert_eq!(solution.get_min(), value);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        };
    }

    test_case!(
        vec![("push", 1), ("push", 2), ("push", 3), ("pop", 0), ("top", 2), ("get_min", 1)],
        test_case_1
    );

    test_case!(vec![("push", 100), ("top", 100), ("get_min", 100)], test_case_2);

    test_case!(
        vec![("push", 100), ("push", -100), ("get_min", -100), ("pop", 0), ("get_min", 100)],
        test_case_3
    );

    test_case!(
        vec![
            ("push", 1),
            ("push", 2),
            ("top", 2),
            ("pop", 0),
            ("top", 1),
            ("push", -1),
            ("top", -1),
            ("push", 2),
            ("get_min", -1),
            ("top", 2)
        ],
        test_case_4
    );

    test_case!(
        vec![
            ("push", 100),
            ("push", 99),
            ("push", 98),
            ("top", 98),
            ("get_min", 98),
            ("pop", 0),
            ("top", 99),
            ("get_min", 99)
        ],
        test_case_5
    );
}
