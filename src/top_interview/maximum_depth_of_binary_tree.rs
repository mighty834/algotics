use crate::common::structs::TreeNode;
use std::cell::RefCell;
/// # Maximum Depth of Binary Tree
///
/// Given the root of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest
/// path from the root node down to the farthest leaf node.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 3
/// ```
///
/// ### Example 2:
/// ```text
/// Input: root = [1,null,2]
/// Output: 2
/// ```
///
/// ## Constraints
///
/// - The number of nodes in the tree is in the range `[0, 10^4]`.
/// - `-100 <= Node.val <= 100`
use std::rc::Rc;
pub struct Solutions;

impl Solutions {
    /// Returns the maximum depth (height) of the binary tree rooted at `root`.
    ///
    /// **Idea (recursive DFS)**
    /// - If the tree is empty (`None`), the depth is `0`.
    /// - Otherwise, recursively compute the maximum depth of the left and right subtrees
    ///   and take the maximum.
    ///
    /// This implementation uses an inner helper `layers_count(node, cx)` that carries the
    /// current depth `cx` while descending. Each `TreeNode` is accessed by borrowing the
    /// `Rc<RefCell<TreeNode>>` (`borrow()`), then recursing on `left`/`right`.
    ///
    /// # Arguments
    /// - `root`: Root node of the tree, represented as `Option<Rc<RefCell<TreeNode>>>`.
    ///
    /// # Returns
    /// The number of nodes along the longest path from the root down to a leaf node.
    ///
    /// # Complexity
    /// - Time: `O(n)` where `n` is the number of nodes (each node visited once)
    /// - Extra space: `O(h)` for the recursion stack, where `h` is the tree height
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        fn layers_count(node: &Option<Rc<RefCell<TreeNode>>>, cx: i32) -> i32 {
            let node = node.as_ref().unwrap().borrow();

            let left: i32 = if node.left.is_some() { layers_count(&node.left, cx + 1) } else { cx };

            let right: i32 =
                if node.right.is_some() { layers_count(&node.right, cx + 1) } else { cx };

            left.max(right)
        }

        layers_count(&root, 1)
    }
}

#[cfg(test)]
mod maximum_depth_of_binary_tree_tests {
    use super::*;

    const NONE: i32 = -101;

    macro_rules! test_case {
        ($root: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let root: Option<Rc<RefCell<TreeNode>>> = $root;
                let expected: i32 = $expected;

                let result: i32 = Solutions::max_depth(root);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(
        TreeNode::build_from_int_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], NONE),
        4,
        test_case_1
    );

    test_case!(TreeNode::build_from_int_vec(vec![1, NONE, 2, 3, 4, 5], NONE), 4, test_case_2);

    test_case!(TreeNode::build_from_vec(vec![Some(1)]), 1, test_case_3);

    test_case!(
        TreeNode::build_from_int_vec(vec![3, 9, 20, NONE, NONE, 15, 7], NONE),
        3,
        test_case_4
    );

    test_case!(TreeNode::build_from_int_vec(vec![1, NONE, 2], NONE), 2, test_case_5);

    test_case!(
        TreeNode::build_from_int_vec(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], NONE),
        5,
        test_case_6
    );

    test_case!(TreeNode::build_from_vec(vec![]), 0, test_case_7);
}
