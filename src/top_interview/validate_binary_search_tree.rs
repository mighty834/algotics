/// # Validate Binary Search Tree
///
/// Given the root of a binary tree, determine if it is a valid binary search tree (BST).
///
/// A valid BST is defined as follows:
///
/// - The left subtree of a node contains only nodes with keys **strictly less** than the node's key.
/// - The right subtree of a node contains only nodes with keys **strictly greater** than the node's key.
/// - Both the left and right subtrees must also be binary search trees.
///
/// ## Examples
///
/// ### Example 1
/// ```text
/// Input: root = [2,1,3]
/// Output: true
/// ```
///
/// ### Example 2
/// ```text
/// Input: root = [5,1,4,null,null,3,6]
/// Output: false
/// Explanation: The root node's value is 5 but its right child's value is 4.
/// ```
///
/// ## Constraints
///
/// - Node count `n` satisfies `1 <= n <= 10^4`.
/// - `-2^31 + 1 <= Node.val <= 2^31 - 1`
pub struct Solutions;

use crate::common::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solutions {
    fn left_check(node: &Option<Rc<RefCell<TreeNode>>>, value: i32) -> bool {
        if node.is_none() {
            return true;
        }

        let val = node.as_ref().unwrap().borrow().val;
        if val >= value {
            return false;
        }

        Self::left_check(&node.as_ref().unwrap().borrow().left.clone(), value)
            && Self::left_check(&node.as_ref().unwrap().borrow().right.clone(), value)
    }

    fn right_check(node: &Option<Rc<RefCell<TreeNode>>>, value: i32) -> bool {
        if node.is_none() {
            return true;
        }

        let val = node.as_ref().unwrap().borrow().val;
        if val <= value {
            return false;
        }

        Self::right_check(&node.as_ref().unwrap().borrow().left.clone(), value)
            && Self::right_check(&node.as_ref().unwrap().borrow().right.clone(), value)
    }

    /// Returns `true` if the binary tree rooted at `root` is a valid BST.
    ///
    /// **Idea (subtree scan + recursion)**
    /// - For the current node with value `v`:
    ///   - verify **every** node in the left subtree is `< v`
    ///   - verify **every** node in the right subtree is `> v`
    /// - If both checks pass, recursively validate the left and right subtrees.
    ///
    /// This implementation performs the “every node in subtree satisfies bound” checks via
    /// the helper functions `left_check` and `right_check`, then calls `is_valid_bst` again
    /// on the left and right children.
    ///
    /// # Arguments
    /// - `root`: Root of the tree (`None` means an empty tree).
    ///
    /// # Returns
    /// `true` if the tree satisfies BST ordering with **strict** inequalities everywhere;
    /// otherwise `false`.
    ///
    /// # Complexity
    /// - Time: worst-case `O(n^2)` (each node may rescan large subtrees; e.g. a skewed tree)
    /// - Extra space: `O(h)` recursion depth, where `h` is the tree height
    ///
    /// # Notes
    /// - Uses `Rc<RefCell<TreeNode>>`; each node value/child pointer is read via `borrow()`.
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let val = root.as_ref().unwrap().borrow().val;

        if !(Self::left_check(&root.as_ref().unwrap().borrow().left, val)
            && Self::right_check(&root.as_ref().unwrap().borrow().right, val))
        {
            return false;
        }

        let root = root.unwrap();
        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        Self::is_valid_bst(left) && Self::is_valid_bst(right)
    }
}

#[cfg(test)]
mod validate_binary_search_tree_tests {
    use super::*;

    const NONE: i32 = i32::MIN;

    macro_rules! test_case {
        ($root: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::build_from_int_vec($root, NONE);
                let expected: bool = $expected;

                let result: bool = Solutions::is_valid_bst(root);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], false, test_case_1);
    test_case!(vec![2, 1, 3], true, test_case_2);
    test_case!(vec![5, 1, 4, NONE, NONE, 3, 6], false, test_case_3);
    test_case!(vec![1, -100, 200], true, test_case_4);
    test_case!(vec![1], true, test_case_5);
    test_case!(vec![3, 2, 4, NONE, NONE, 5, 6, NONE, 8, 9, 10], false, test_case_6);
    test_case!(vec![1, 1, 1], false, test_case_7);
    test_case!(vec![5, 4, 6, NONE, NONE, 3, 7], false, test_case_8);
}
