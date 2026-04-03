/// # Reverse Linked List
///
/// Given the head of a singly linked list, reverse the list,
/// and return the reversed list.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: head = [1,2,3,4,5]
/// Output: [5,4,3,2,1]
/// ```
///
/// ### Example 2:
/// ```text
/// Input: head = [1,2]
/// Output: [2,1]
/// ```
///
/// ### Example 3:
/// ```text
/// Input: head = []
/// Output: []
/// ```
///
/// ## Constraints
///
/// - The number of nodes in the list is in the range `[0, 5000]`.
/// - `-5000 <= Node.val <= 5000`
///
/// ## Follow-up
///
/// A linked list can be reversed either iteratively or recursively.
/// Could you implement both?
use crate::common::structs::ListNode;
pub struct Solutions;

impl Solutions {
    /// Reverses a singly linked list and returns the new head.
    ///
    /// **Idea**
    /// - Walk the list from front to back and push each `val` into a `Vec<i32>`.
    /// - Rebuild a fresh list by folding over the vector: for each value, allocate a new
    ///   [`ListNode`] whose `next` is the list built so far. Because new nodes are prepended
    ///   in **forward** vector order, the final chain is the original order reversed.
    ///
    /// This does not flip pointers in the original nodes; it **consumes** the input list
    /// (via `unwrap` while draining `next`) and constructs an entirely new reversed list.
    ///
    /// # Arguments
    /// - `head`: Head of the list, or `None` for an empty list.
    ///
    /// # Returns
    /// Head of the reversed list, or `None` if the input was empty.
    ///
    /// # Complexity
    /// - Time: `O(n)` for `n` nodes (one traversal + one rebuild)
    /// - Extra space: `O(n)` for the vector and the new boxed nodes
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nodes: Vec<i32> = Vec::new();
        let mut node = head;

        while node.is_some() {
            nodes.push(node.as_ref()?.val);
            node = node.unwrap().next;
        }

        nodes.into_iter().fold(None, |node, val| {
            let prev_node = Box::new(ListNode { val, next: node });
            Some(prev_node)
        })
    }
}

#[cfg(test)]
mod reverse_linked_list_tests {
    use super::*;

    macro_rules! test_case {
        ($head: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let head: Option<Box<ListNode>> = $head;
                let expected: Option<Box<ListNode>> = $expected;

                let result = Solutions::reverse_list(head);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(
        ListNode::build_from_vec(vec![1, 2, 3, 4, 5]),
        ListNode::build_from_vec(vec![5, 4, 3, 2, 1]),
        test_case_1
    );

    test_case!(ListNode::build_from_vec(vec![]), ListNode::build_from_vec(vec![]), test_case_2);

    test_case!(ListNode::build_from_vec(vec![1]), ListNode::build_from_vec(vec![1]), test_case_3);

    test_case!(
        ListNode::build_from_vec(vec![-100, -200, -300, -400, -500]),
        ListNode::build_from_vec(vec![-500, -400, -300, -200, -100]),
        test_case_4
    );

    test_case!(
        ListNode::build_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        ListNode::build_from_vec(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
        test_case_5
    );

    test_case!(
        ListNode::build_from_vec(vec![67, 23, -1]),
        ListNode::build_from_vec(vec![-1, 23, 67]),
        test_case_6
    );
}
