use std::collections::HashSet;

/// Determines whether a partially filled `9 x 9` Sudoku board is valid.
///
/// Only the filled cells need to be validated according to the following rules:
///
/// - Each row must contain the digits `1` through `9` without repetition
/// - Each column must contain the digits `1` through `9` without repetition
/// - Each of the nine `3 x 3` sub-boxes must contain the digits `1` through `9`
///   without repetition
///
/// Empty cells are represented by `'.'`.
///
/// # Notes
///
/// - A valid board is not necessarily solvable
/// - Only the currently filled cells must be checked
///
/// # Examples
///
/// ```text
/// Input:
/// board =
/// [
///   ["5","3",".",".","7",".",".",".","."],
///   ["6",".",".","1","9","5",".",".","."],
///   [".","9","8",".",".",".",".","6","."],
///   ["8",".",".",".","6",".",".",".","3"],
///   ["4",".",".","8",".","3",".",".","1"],
///   ["7",".",".",".","2",".",".",".","6"],
///   [".","6",".",".",".",".","2","8","."],
///   [".",".",".","4","1","9",".",".","5"],
///   [".",".",".",".","8",".",".","7","9"]
/// ]
/// Output: true
/// ```
///
/// ```text
/// Input:
/// board =
/// [
///   ["8","3",".",".","7",".",".",".","."],
///   ["6",".",".","1","9","5",".",".","."],
///   [".","9","8",".",".",".",".","6","."],
///   ["8",".",".",".","6",".",".",".","3"],
///   ["4",".",".","8",".","3",".",".","1"],
///   ["7",".",".",".","2",".",".",".","6"],
///   [".","6",".",".",".",".","2","8","."],
///   [".",".",".","4","1","9",".",".","5"],
///   [".",".",".",".","8",".",".","7","9"]
/// ]
/// Output: false
/// Explanation: The board is invalid because the top-left `3 x 3` sub-box
/// contains two `'8'` values.
/// ```
///
/// # Constraints
///
/// - `board.len() == 9`
/// - `board[i].len() == 9`
/// - `board[i][j]` is a digit `'1'..='9'` or `'.'`
pub struct Solutions;

impl Solutions {
    /// Validate a 9×9 Sudoku board by checking rows, columns, and 3×3 boxes in one pass.
    ///
    /// **Idea**
    /// - Maintain nine [`HashSet`]s per **row**, nine per **column**,
    ///   and nine per **3×3 block** (`raw_hashes`, `col_hashes`, `box_hashes`).
    /// - For each cell that is not `'.'`, insert the digit into the three corresponding sets.
    /// - [`HashSet::insert`] returns `false` if the value was already present — that means a
    ///   duplicate in that row, column, or box, so the board is invalid.
    ///
    /// Empty cells are skipped; only filled digits participate in uniqueness checks.
    ///
    /// **Box index**
    /// `get_box_hash_index(i, j)` maps `(i, j)` to one of `0..9` via an explicit match (equivalent
    /// to the usual `(i / 3) * 3 + (j / 3)` for a 9×9 grid).
    ///
    /// # Complexity
    ///
    /// - Time: **O(1)** for a fixed 9×9 board (81 cells; constant work per cell).
    /// - Extra space: **O(1)** for a fixed board — bounded by the number of digit slots per
    ///   row/column/box.
    ///
    /// # Panics
    ///
    /// The inner helper `get_box_hash_index` panics with `"Invalid sudoku coordinates"` if `(i, j)` is outside
    /// `0..9` (the implementation assumes a well-formed 9×9 `board`).
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        fn get_box_hash_index(i: usize, j: usize) -> usize {
            match (i, j) {
                (0, 0) | (0, 1) | (0, 2) | (1, 0) | (1, 1) | (1, 2) | (2, 0) | (2, 1) | (2, 2) => 0,
                (0, 3) | (0, 4) | (0, 5) | (1, 3) | (1, 4) | (1, 5) | (2, 3) | (2, 4) | (2, 5) => 1,
                (0, 6) | (0, 7) | (0, 8) | (1, 6) | (1, 7) | (1, 8) | (2, 6) | (2, 7) | (2, 8) => 2,
                (3, 0) | (3, 1) | (3, 2) | (4, 0) | (4, 1) | (4, 2) | (5, 0) | (5, 1) | (5, 2) => 3,
                (3, 3) | (3, 4) | (3, 5) | (4, 3) | (4, 4) | (4, 5) | (5, 3) | (5, 4) | (5, 5) => 4,
                (3, 6) | (3, 7) | (3, 8) | (4, 6) | (4, 7) | (4, 8) | (5, 6) | (5, 7) | (5, 8) => 5,
                (6, 0) | (6, 1) | (6, 2) | (7, 0) | (7, 1) | (7, 2) | (8, 0) | (8, 1) | (8, 2) => 6,
                (6, 3) | (6, 4) | (6, 5) | (7, 3) | (7, 4) | (7, 5) | (8, 3) | (8, 4) | (8, 5) => 7,
                (6, 6) | (6, 7) | (6, 8) | (7, 6) | (7, 7) | (7, 8) | (8, 6) | (8, 7) | (8, 8) => 8,
                _ => panic!("Invalid sudoku coordinates"),
            }
        }

        let mut raw_hashes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut col_hashes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut box_hashes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let current_value: char = if board[i][j] == '.' { continue } else { board[i][j] };

                if !raw_hashes[i].insert(current_value) ||
                   !col_hashes[j].insert(current_value) ||
                   !box_hashes[get_box_hash_index(i, j)].insert(current_value) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod valid_sudoku_tests {
    use super::*;

    macro_rules! test_case {
        ($board: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let board: Vec<Vec<char>> = $board;
                let expected: bool = $expected;

                let result: bool = Solutions::is_valid_sudoku(board);
                assert_eq!(result, expected);
            }
        }
    }

    test_case!(
        vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ], true, test_case_1);

    test_case!(
        vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ], false, test_case_2);

    test_case!(
        vec![
            vec!['1','2','3','4','5','6','7','8','9'],
            vec!['4','.','.','.','.','.','.','.','.'],
            vec!['5','.','.','.','.','.','.','.','.'],
            vec!['9','.','.','.','.','.','.','.','.'],
            vec!['2','.','.','.','.','.','.','.','.'],
            vec!['3','.','.','.','.','.','.','.','.'],
            vec!['7','.','.','.','.','.','.','.','.'],
            vec!['8','.','.','.','.','.','.','.','.'],
            vec!['6','.','.','.','.','.','.','.','.'],
        ], true, test_case_3);

    test_case!(
        vec![
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
        ], true, test_case_4);

    test_case!(
        vec![
            vec!['1','2','3','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','4','5','6','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','7','8','9','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','4','5','6','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','1','2','3'],
        ], true, test_case_5);

    test_case!(
        vec![
            vec!['1','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','2','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','1','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','1'],
        ], false, test_case_6);
}