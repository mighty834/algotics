/// Rotates an `n × n` 2D matrix (image) 90° clockwise.
///
/// The input matrix represents an image, where each element corresponds to a pixel value.
/// The public API overwrites the input `matrix` with the rotated result; see
/// [`Solutions::rotate`] for how the reference solution in this file is built.
///
/// # Examples
///
/// ```text
/// Input: matrix = [[1,2,3],
///                  [4,5,6],
///                  [7,8,9]]
/// Output: [[7,4,1],
///          [8,5,2],
///          [9,6,3]]
/// ```
///
/// ```text
/// Input: matrix = [[5,1,9,11],
///                  [2,4,8,10],
///                  [13,3,6,7],
///                  [15,14,12,16]]
/// Output: [[15,13,2,5],
///          [14,3,4,1],
///          [12,6,8,9],
///          [16,7,10,11]]
/// ```
///
/// # Constraints
///
/// - `n == matrix.len() == matrix[i].len()`
/// - `1 <= n <= 20`
/// - `-1000 <= matrix[i][j] <= 1000`
///
/// # Problem requirements (typical statement)
///
/// - Rotate the image 90° clockwise.
/// - Many versions also ask for **O(1)** extra space (no second `n × n` buffer). The
///   implementation of [`Solutions::rotate`] in this file uses a temporary
///   matrix for a straightforward build-then-copy approach instead.
///
/// # Notes
///
/// - The matrix is square.
/// - Rotation is clockwise by 90 degrees.
pub struct Solutions;

impl Solutions {
    /// Rotates a square `n × n` matrix 90° clockwise, updating `matrix` in place.
    ///
    /// This is a “cheat” / direct construction: it allocates a full auxiliary `n × n`
    /// buffer, fills it with the rotated values, then copies each row back into `matrix`
    /// with `Vec::clone_from_slice`. Row `i` of the output reads **column** `i` of the
    /// input from bottom to top, which yields `result[i][j] = matrix[n - 1 - j][i]` — the
    /// standard 90° clockwise rotation of a square matrix.
    ///
    /// # Complexity
    ///
    /// - **Time:** `O(n²)` — every cell is read and written a constant number of times.
    /// - **Extra space:** `O(n²)` for the temporary matrix (unlike an in-layer rotation
    ///   that only uses `O(1)` extra variables).
    ///
    /// # Arguments
    ///
    /// * `matrix` — Square grid: `matrix.len() == n` and every row should have length `n`.
    ///   After the call, `matrix[r][c]` is the value that was at the position that moves
    ///   to `(r, c)` under 90° clockwise rotation.
    ///
    /// # Panics
    ///
    /// If the matrix is not square (a row length differs from `matrix.len()`), indexing
    /// may panic or the result may be wrong; this function expects a valid `n × n` input.
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut result: Vec<Vec<i32>> = vec![Vec::with_capacity(matrix.len()); matrix.len()];

        for i in 0..matrix.len() {
            for j in (0..matrix.len()).rev() {
                result[i].push(matrix[j][i]);
            }
        }

        for i in 0..matrix.len() {
            matrix[i].clone_from_slice(&result[i]);
        }
    }
}

#[cfg(test)]
mod rotate_image_tests {
    use super::*;

    macro_rules! test_case {
        ($matrix: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let mut matrix: Vec<Vec<i32>> = $matrix;
                let expected: Vec<Vec<i32>> = $expected;

                Solutions::rotate(&mut matrix);
                assert_eq!(matrix, expected);
            }
        }
    }

    test_case!(
        vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ],
        vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3]
        ], test_case_1);

    test_case!(
        vec![
            vec![1, 2],
            vec![3, 4]
        ],
        vec![
            vec![3, 1],
            vec![4, 2]
        ], test_case_2);

    test_case!(vec![vec![1]], vec![vec![1]], test_case_3);

    test_case!(
        vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ],
        vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4]
        ], test_case_4);

    test_case!(
        vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25]
        ],
        vec![
            vec![21, 16, 11, 6, 1],
            vec![22, 17, 12, 7, 2],
            vec![23, 18, 13, 8, 3],
            vec![24, 19, 14, 9, 4],
            vec![25, 20, 15, 10, 5]
        ], test_case_5);
}
