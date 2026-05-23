/// # Binary Tree Level Order Traversal
///
/// Given the root of a binary tree, return the level order traversal
/// of its nodes' values.
///
/// Level order traversal visits nodes:
///
/// - From left to right
/// - Level by level
///
/// Example traversal:
///
/// ```text
///       3
///      / \
///     9  20
///       /  \
///      15   7
///
/// Result:
/// [
///     [3],
///     [9, 20],
///     [15, 7]
/// ]
/// ```
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [[3],[9,20],[15,7]]
/// ```
///
/// ### Example 2:
/// ```text
/// Input: root = [1]
/// Output: [[1]]
/// ```
///
/// ### Example 3:
/// ```text
/// Input: root = []
/// Output: []
/// ```
///
/// ## Constraints
///
/// - `0 <= number_of_nodes <= 2000`
/// - `-1000 <= Node.val <= 1000`
///
/// ## Notes
///
/// A common solution uses:
///
/// - Breadth-First Search (BFS)
/// - A queue to process nodes level by level
///
/// Example queue state:
///
/// ```text
/// Queue:
/// [3]
///
/// Queue:
/// [9,20]
///
/// Queue:
/// [15,7]
/// ```
///
/// ## Complexity
///
/// BFS solution:
///
/// - Time: `O(n)`
/// - Space: `O(n)`
pub struct Solutions;

use crate::common::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solutions {
    /// Return level-order traversal of the binary tree rooted at `root`.
    ///
    /// Each inner vector holds node values at one depth, left to right (BFS order).
    /// An empty tree yields an empty outer vector.
    ///
    /// # Arguments
    ///
    /// * `root` — Tree root as `Option<Rc<RefCell<TreeNode>>>`, the usual LeetCode-style
    ///   representation in this crate.
    ///
    /// # Returns
    ///
    /// `result[i]` is the list of values at depth `i` (root at depth `0`).
    ///
    /// # Algorithm
    ///
    /// Recursive **level-by-level** pass via inner helper `req`:
    ///
    /// 1. For each node in the current level, `borrow_mut()` once, read `val`, then move
    ///    children into the next level with [`Option::take`] on `left` / `right` (no `Rc::clone`).
    /// 2. Append the level's values to `result`.
    /// 3. If any children were taken, recurse on `nnodes`; otherwise return.
    ///
    /// # Ownership note
    ///
    /// [`Option::take`] **moves** each child `Rc` out of its parent and leaves `None` in the
    /// parent field. The tree is dismantled as the walk proceeds — fine for a one-shot traversal
    /// that only needs the level lists, but unsuitable if the caller must reuse `root` afterward.
    ///
    /// # Examples
    ///
    /// ```text
    /// Input: root = [3,9,20,null,null,15,7]
    /// Output: [[3],[9,20],[15,7]]
    /// ```
    ///
    /// ```text
    /// Input: root = []
    /// Output: []
    /// ```
    ///
    /// # Complexity
    ///
    /// - Time: **O(n)** — each node is processed once.
    /// - Extra space: **O(n)** for the output plus **O(width)** for the next-level `nnodes`
    ///   vector (and recursion depth bounded by tree height).
    ///
    /// # Panics
    ///
    /// Panics if `req` receives `None` in its node list (`unwrap()` on `node`). With a
    /// well-formed tree built from the test helper, only `Some` nodes are ever enqueued.
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        fn req(
            nodes: Vec<Option<Rc<RefCell<TreeNode>>>>,
            mut result: Vec<Vec<i32>>,
        ) -> Vec<Vec<i32>> {
            let mut values: Vec<i32> = Vec::new();
            let mut nnodes: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

            for node in nodes {
                let node = node.unwrap();
                let mut n = node.borrow_mut();

                values.push(n.val);

                if let Some(left) = n.left.take() {
                    nnodes.push(Some(left))
                }

                if let Some(right) = n.right.take() {
                    nnodes.push(Some(right))
                }
            }
            result.push(values);

            if nnodes.len() > 0 {
                return req(nnodes, result);
            }

            result
        }

        let result: Vec<Vec<i32>> = Vec::new();
        req(vec![root], result)
    }
}

#[cfg(test)]
mod binary_tree_level_order_traversal_tests {
    use super::*;

    #[allow(non_upper_case_globals)]
    const null: i32 = i32::MIN;

    macro_rules! test_case {
        ($root: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::build_from_int_vec($root, null);
                let expected: Vec<Vec<i32>> = $expected;

                let result: Vec<Vec<i32>> = Solutions::level_order(root);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(
        vec![3, 9, 20, null, null, 15, 7],
        vec![vec![3], vec![9, 20], vec![15, 7]],
        test_case_1
    );

    test_case!(vec![1], vec![vec![1]], test_case_2);

    test_case!(vec![], vec![], test_case_3);

    test_case!(
        vec![3, 9, 20, 21, 22, 23, 15, 7, null, null, 43, 44],
        vec![vec![3], vec![9, 20], vec![21, 22, 23, 15], vec![7, 43, 44]],
        test_case_4
    );

    test_case!(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        vec![vec![1], vec![2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11, 12]],
        test_case_5
    );

    test_case!(
        vec![1, null, 2, 3, null, null, 4, null, 5],
        vec![vec![1], vec![2], vec![3], vec![4], vec![5]],
        test_case_6
    );
}
