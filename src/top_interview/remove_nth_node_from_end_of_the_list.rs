/// Removes the n-th node from the end of a singly linked list and returns the new head.
///
/// Given the head of a linked list `head` and an integer `n`, removes the n-th node
/// from the end of the list and returns the updated head.
///
/// # Examples
///
/// ```text
/// Input: head = [1,2,3,4,5], n = 2
/// Output: [1,2,3,5]
/// ```
///
/// ```text
/// Input: head = [1], n = 1
/// Output: []
/// ```
///
/// ```text
/// Input: head = [1,2], n = 1
/// Output: [1]
/// ```
///
/// # Constraints
///
/// - The number of nodes in the list is `sz`
/// - `1 <= sz <= 30`
/// - `0 <= Node.val <= 100`
/// - `1 <= n <= sz`
///
/// # Requirements
///
/// - Remove exactly one node: the n-th node from the end
/// - Return the head of the modified list
///
/// # Follow-up
///
/// - Can this be done in a single pass over the list?
///
/// # Notes
///
/// - The list is singly linked
/// - Removing the head node should return the next node as the new head
use crate::common::structs::ListNode;
pub struct Solutions;

impl Solutions {
    /// Removes the n-th node from the end by materializing the list into a `Vec`.
    ///
    /// **How this implementation works**
    /// - Traverse the list and collect node values into `nodes`.
    /// - Compute the index to remove from the front: `nodes.len() - n`.
    /// - Remove that element from the vector.
    /// - Rebuild a new linked list from the remaining values (fold from the back).
    ///
    /// This approach is simple, but it does **not** modify the original nodes in-place: it
    /// constructs a fresh list containing the same remaining values.
    ///
    /// # Arguments
    /// - `head`: Head of the singly linked list.
    /// - `n`: 1-based position from the end to remove.
    ///
    /// # Returns
    /// The head of the list after removing the n-th node from the end.
    ///
    /// # Complexity
    /// - Time: `O(sz)` where `sz` is the number of nodes (one traversal + rebuild)
    /// - Extra space: `O(sz)` for the vector and rebuilt nodes
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut nodes: Vec<i32> = Vec::new();

        let mut node = head;
        while node.is_some() {
            nodes.push(node.as_ref()?.val);
            node = node.and_then(|n| n.next);
        }

        let drain_index: usize = nodes.len() - (n as usize);
        nodes.remove(drain_index);

        nodes
            .into_iter()
            .rev()
            .fold(None, |node, val| {
                let mut prev_node: Box<ListNode> = Box::new(ListNode::new(val));
                prev_node.next = node;
                Some(prev_node)
            })
    }

    /// Removes the n-th node from the end using two passes over the list.
    ///
    /// **Idea (two-pass by length)**
    /// - First pass: compute the list length `sz`.
    /// - Convert “n-th from the end” into a 0-based index from the front:
    ///   `index_for_change = sz - n`.
    /// - If that index is `0`, the node to remove is the head: return `head.next`.
    /// - Second pass: walk to the node *just before* the target, then bypass the target by
    ///   rewiring `next` pointers (`curr.next = curr.next.next`).
    ///
    /// This modifies the existing list structure (no full rebuild).
    ///
    /// # Arguments
    /// - `head`: Head of the singly linked list.
    /// - `n`: 1-based position from the end to remove.
    ///
    /// # Returns
    /// The head of the list after removing the n-th node from the end.
    ///
    /// # Complexity
    /// - Time: `O(sz)` (two linear passes)
    /// - Extra space: `O(1)`
    pub fn remove_nth_from_end_2_pass(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut size: i32 = 0;
        let mut node = &head;
        while node.is_some() {
            size += 1;
            node = &node.as_ref()?.next;
        }

        let index_for_change = size - n;
        if index_for_change == 0 {
            return head?.next;
        }

        let mut node = head;
        let mut curr = &mut node;
        for _ in 1..index_for_change {
            curr = &mut curr.as_mut()?.next;
        }

        let mut next = &mut curr.as_mut()?.next;
        curr.as_mut()?.next = next.as_mut()?.next.take();

        node
    }
}

#[cfg(test)]
mod remove_nth_node_from_end_of_the_list_tests {
    use super::*;

    macro_rules! test_case {
        ($head: expr, $n: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let head: Option<Box<ListNode>> = $head;
                let n: i32 = $n;
                let expected: Option<Box<ListNode>> = $expected;

                let result: Option<Box<ListNode>> = Solutions::remove_nth_from_end(head, n);
                assert_eq!(result, expected);

                let head: Option<Box<ListNode>> = $head;
                let result: Option<Box<ListNode>> = Solutions::remove_nth_from_end_2_pass(head, n);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(
        ListNode::build_from_vec(vec![1, 2, 3, 4]),
        1,
        ListNode::build_from_vec(vec![1, 2, 3]),
        test_case_1
    );

    test_case!(
        ListNode::build_from_vec(vec![1]),
        1,
        ListNode::build_from_vec(vec![]),
        test_case_2
    );

    test_case!(
        ListNode::build_from_vec(vec![5, 4, 3, 2, 1]),
        5,
        ListNode::build_from_vec(vec![4, 3, 2, 1]),
        test_case_3
    );

    test_case!(
        ListNode::build_from_vec(vec![-1, -2, -3, -4, -5]),
        3,
        ListNode::build_from_vec(vec![-1, -2, -4, -5]),
        test_case_4
    );

    test_case!(
        ListNode::build_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        4,
        ListNode::build_from_vec(vec![1, 2, 3, 4, 5, 6, 8, 9, 10]),
        test_case_5
    );

    test_case!(
        ListNode::build_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        9,
        ListNode::build_from_vec(vec![1, 3, 4, 5, 6, 7, 8, 9, 10]),
        test_case_6
    );
}
