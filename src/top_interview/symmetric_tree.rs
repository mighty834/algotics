/// # Symmetric Tree
///
/// Given the root of a binary tree, check whether it is a mirror of itself
/// (i.e., symmetric around its center).
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: root = [1,2,2,3,4,4,3]
/// Output: true
/// ```
///
/// ### Example 2:
/// ```text
/// Input: root = [1,2,2,null,3,null,3]
/// Output: false
/// ```
///
/// ## Constraints
///
/// - The number of nodes in the tree is in the range `[1, 1000]`.
/// - `-100 <= Node.val <= 100`
///
/// ## Follow-up
///
/// Could you solve it both recursively and iteratively?
pub struct Solutions;

use crate::common::structs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solutions {
    /// Returns whether the binary tree rooted at `root` is symmetric: its left and right
    /// subtrees are mirrors of each other (same shape and values when reflected across the
    /// vertical axis through the root).
    ///
    /// This compares the left child of the root with the right child recursively: two nodes
    /// match if their values are equal, the left subtree of the first mirrors the right subtree
    /// of the second, and the right subtree of the first mirrors the left subtree of the second.
    /// Empty subtrees on both sides match.
    ///
    /// # Arguments
    ///
    /// * `root` — The root of the tree. Must be [`Some`]; an empty tree is not accepted here
    ///   (the LeetCode constraints guarantee at least one node).
    ///
    /// # Returns
    ///
    /// `true` if the tree is symmetric, `false` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if `root` is `None`, because the implementation unwraps the root to read its
    /// left and right children.
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn rec_checking(
            node_1: &Option<Rc<RefCell<TreeNode>>>,
            node_2: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if node_1.is_none() && node_2.is_none() {
                return true;
            }

            if node_1.is_some() && node_2.is_some()
                && node_1.as_ref().unwrap().borrow().val == node_2.as_ref().unwrap().borrow().val {
                    return rec_checking(
                        &node_1.as_ref().unwrap().borrow().left,
                        &node_2.as_ref().unwrap().borrow().right,
                    ) && rec_checking(
                        &node_1.as_ref().unwrap().borrow().right,
                        &node_2.as_ref().unwrap().borrow().left,
                    );
                }

            false
        }

        rec_checking(&root.as_ref().unwrap().borrow().left, &root.as_ref().unwrap().borrow().right)
    }
}

#[cfg(test)]
mod symmetric_tree_tests {
    use super::*;

    #[allow(non_upper_case_globals)]
    const null: i32 = i32::MIN;

    macro_rules! test_case {
        ($root: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let root: Option<Rc<RefCell<TreeNode>>> = TreeNode::build_from_int_vec($root, null);
                let expected: bool = $expected;

                let result = Solutions::is_symmetric(root);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![1, 2, 2, 3, 4, 4, 3], true, test_case_1);
    test_case!(vec![1, 2, 2, null, 3, null, 3], false, test_case_2);
    test_case!(vec![1, 2, 2, 3, 4, 4, 3, null, 1, 2, 3, 3, 2, 1, null], true, test_case_3);
    test_case!(vec![1], true, test_case_4);
    test_case!(vec![-100, -200, -200, -300, -400, -400, -300], true, test_case_5);
    test_case!(vec![1, null, 2, 3, 4, null, 1], false, test_case_6);
    test_case!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], false, test_case_7);
    test_case!(vec![1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 1, 4], false, test_case_8);
}
