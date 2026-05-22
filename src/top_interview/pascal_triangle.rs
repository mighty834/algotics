/// # Pascal's Triangle
///
/// Given an integer `num_rows`, return the first `num_rows` of Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly
/// above it.
///
/// Example visualization:
///
/// ```text
///            1
///          1   1
///        1   2   1
///      1   3   3   1
///    1   4   6   4   1
/// ```
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: num_rows = 5
/// Output:
/// [
///     [1],
///     [1,1],
///     [1,2,1],
///     [1,3,3,1],
///     [1,4,6,4,1]
/// ]
/// ```
///
/// ### Example 2:
/// ```text
/// Input: num_rows = 1
/// Output:
/// [
///     [1]
/// ]
/// ```
///
/// ## Constraints
///
/// - `1 <= num_rows <= 30`
pub struct Solutions;

impl Solutions {
    /// Build the first `num_rows` rows of Pascal's triangle.
    ///
    /// Row `r` (1-based) has `r` entries; the first and last are always `1`, and each interior
    /// value is the sum of the two cells directly above it in the previous row.
    ///
    /// # Arguments
    ///
    /// * `num_rows` — How many rows to return. Expected in `1..=30` per problem constraints.
    ///
    /// # Returns
    ///
    /// A vector of rows, where `result[i]` is row `i + 1` (e.g. `result[0] == [1]`).
    ///
    /// # Algorithm
    ///
    /// 1. **Base cases:** `num_rows == 1` → `[[1]]`; `num_rows == 2` → `[[1], [1, 1]]`.
    /// 2. **Seed** `result` with those first two rows.
    /// 3. For each `i` in `3..=num_rows`, build a new row from the previous row `last_row`:
    ///    - Push leading `1`.
    ///    - For `j` in `1..i - 1`, push `last_row[j - 1] + last_row[j]`.
    ///    - Push trailing `1` and append to `result`.
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: num_rows = 5
    /// Output: [[1], [1,1], [1,2,1], [1,3,3,1], [1,4,6,4,1]]
    /// ```
    ///
    /// ```text
    /// Input: num_rows = 1
    /// Output: [[1]]
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(num_rows²)** — row `i` has `i` cells and the total cell count is
    ///   `1 + 2 + … + num_rows`.
    /// - Extra space: **O(num_rows²)** for the returned triangle (excluding the output, only
    ///   one previous row is held at a time).
    ///
    /// # Panics
    ///
    /// Panics if `result.last()` is `None` inside the main loop (unreachable when
    /// `num_rows >= 3` because two seed rows are always present).
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 1 {
            return vec![vec![1]];
        }

        if num_rows == 2 {
            return vec![vec![1], vec![1, 1]];
        }

        let mut result: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
        for i in 3..=num_rows {
            let last_row = result.last().unwrap();
            let mut row: Vec<i32> = Vec::with_capacity(i as usize);
            row.push(1);

            for j in 1..i - 1 {
                row.push(last_row[(j - 1) as usize] + last_row[j as usize]);
            }
            row.push(1);

            result.push(row)
        }

        result
    }
}

#[cfg(test)]
mod pascal_triangle_tests {
    use super::*;

    macro_rules! test_case {
        ($num_rows: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let num_rows: i32 = $num_rows;
                let expected: Vec<Vec<i32>> = $expected;

                let result: Vec<Vec<i32>> = Solutions::generate(num_rows);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(1, vec![vec![1]], test_case_1);
    test_case!(2, vec![vec![1], vec![1, 1]], test_case_2);
    test_case!(3, vec![vec![1], vec![1, 1], vec![1, 2, 1]], test_case_3);
    test_case!(4, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]], test_case_4);
    test_case!(
        5,
        vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]],
        test_case_5
    );
}
