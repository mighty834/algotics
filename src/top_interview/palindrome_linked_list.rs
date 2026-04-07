/// # Palindrome Linked List
///
/// Given the head of a singly linked list, return `true` if it is a palindrome
/// or `false` otherwise.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: head = [1,2,2,1]
/// Output: true
/// ```
///
/// ### Example 2:
/// ```text
/// Input: head = [1,2]
/// Output: false
/// ```
///
/// ## Constraints
///
/// - The number of nodes in the list is in the range `[1, 10^5]`.
/// - `0 <= Node.val <= 9`
///
/// ## Follow-up
///
/// Could you do it in `O(n)` time and `O(1)` space?
use crate::common::structs::ListNode;
pub struct Solutions;

impl Solutions {
    /// Returns `true` if the linked list values form a palindrome.
    ///
    /// **Idea**
    /// - Traverse the list and collect all node values into a `Vec<i32>`.
    /// - Let `half = len / 2`.
    /// - Compare the left half (`nodes[..half]`) with the reversed right half
    ///   (`nodes[len - half..]` reversed).
    ///
    /// This matches the problem definition, but it uses extra memory (it does not implement
    /// the follow-up `O(1)` space approach).
    ///
    /// # Arguments
    /// - `head`: Head of the singly linked list.
    ///
    /// # Returns
    /// `true` if the sequence of values reads the same forward and backward; otherwise
    /// `false`.
    ///
    /// # Complexity
    /// - Time: `O(n)` for `n` nodes
    /// - Extra space: `O(n)` for the collected values (and the temporary reversed half)
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut nodes: Vec<i32> = Vec::new();
        let mut node = head;

        while let Some(n) = node.as_ref() {
            nodes.push(n.val);
            node = node.unwrap().next;
        }

        let half: usize = nodes.len() / 2;
        let left = &nodes[..half];
        let right: Vec<i32> =
            nodes[nodes.len() - half..].into_iter().rev().map(|val| *val).collect();

        left == right
    }
}

#[cfg(test)]
mod palindrome_linked_list_tests {
    use super::*;

    macro_rules! test_case {
        ($head: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let head: Option<Box<ListNode>> = $head;
                let expected: bool = $expected;

                let result: bool = Solutions::is_palindrome(head);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(ListNode::build_from_vec(vec![1, 2, 3, 4, 5]), false, test_case_1);

    test_case!(ListNode::build_from_vec(vec![1, 2, 3, 2, 1]), true, test_case_2);

    test_case!(ListNode::build_from_vec(vec![1]), true, test_case_3);

    test_case!(ListNode::build_from_vec(vec![1, 2]), false, test_case_4);

    test_case!(
        ListNode::build_from_vec(vec![-100, -200, -300, -300, -200, -100]),
        true,
        test_case_5
    );

    test_case!(ListNode::build_from_vec(vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1]), true, test_case_6);

    test_case!(ListNode::build_from_vec(vec![1, 2, 3, 4, 3, 2, 0]), false, test_case_7);

    test_case!(ListNode::build_from_vec(vec![1, 2, 3, 4, 5, 6, 4, 3, 2, 1]), false, test_case_8);
}
