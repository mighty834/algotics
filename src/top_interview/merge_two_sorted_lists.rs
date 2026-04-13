/// # Merge Two Sorted Lists
///
/// You are given the heads of two sorted linked lists `list1` and `list2`.
///
/// Merge the two lists into one sorted list. The list should be made by
/// splicing together the nodes of the first two lists.
///
/// Return the head of the merged linked list.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: list1 = [1,2,4], list2 = [1,3,4]
/// Output: [1,1,2,3,4,4]
/// ```
///
/// ### Example 2:
/// ```text
/// Input: list1 = [], list2 = []
/// Output: []
/// ```
///
/// ### Example 3:
/// ```text
/// Input: list1 = [], list2 = [0]
/// Output: [0]
/// ```
///
/// ## Constraints
///
/// - The number of nodes in both lists is in the range `[0, 50]`.
/// - `-100 <= Node.val <= 100`
/// - Both `list1` and `list2` are sorted in non-decreasing order.
use crate::common::structs::ListNode;
pub struct Solutions;

impl Solutions {
    /// Merges two sorted linked lists into a single sorted list.
    ///
    /// **Idea**
    /// - Traverse `list1` and `list2` by reference and push every `val` into one `Vec<i32>`.
    /// - [`sort`](slice::sort) the vector (non-decreasing).
    /// - Build a new list from the sorted values by folding from the end: each step prepends
    ///   a new [`ListNode`], producing the correct forward order.
    ///
    /// This matches the **sorted output** of merging two sorted lists, but it does **not**
    /// splice the original `Box<ListNode>` chains: the input lists are dropped when the
    /// function returns, and the result is made of **newly allocated** nodes.
    ///
    /// # Arguments
    /// - `list1`: Head of the first sorted list (or `None`).
    /// - `list2`: Head of the second sorted list (or `None`).
    ///
    /// # Returns
    /// Head of a new merged list containing all values from both inputs in sorted order, or
    /// `None` if both inputs were empty.
    ///
    /// # Complexity
    /// - Time: `O((n + m) log(n + m))` for sorting, plus linear scans to collect values
    /// - Extra space: `O(n + m)` for the vector and rebuilt nodes
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut nodes: Vec<i32> = Vec::new();

        let mut node = list1.as_ref();
        while node.is_some() {
            nodes.push(node?.val);
            node = node?.next.as_ref();
        }

        let mut node = list2.as_ref();
        while node.is_some() {
            nodes.push(node?.val);
            node = node?.next.as_ref();
        }

        nodes.sort();

        nodes.into_iter().rev().fold(None, |node, val| {
            let prev_node = Box::new(ListNode { val, next: node });
            Some(prev_node)
        })
    }

    /// Merges two sorted linked lists, collecting values in parallel (one thread) and rebuilding a new list.
    ///
    /// **Idea**
    /// - Spawn a thread to traverse `list1` and collect its values into a `Vec<i32>`.
    /// - Traverse `list2` on the current thread and collect its values.
    /// - Join the thread, append both vectors, then sort the combined values.
    /// - Build a new sorted list by folding from the end (prepending new `ListNode`s).
    ///
    /// This produces the correct sorted merged output, but like
    /// [`merge_two_lists`](Self::merge_two_lists) it does **not** splice existing nodes: it
    /// allocates a brand-new list from the collected values.
    ///
    /// # Arguments
    /// - `list1`: Head of the first sorted list (or `None`).
    /// - `list2`: Head of the second sorted list (or `None`).
    ///
    /// # Returns
    /// Head of a new merged list containing all values from both inputs in sorted order.
    ///
    /// # Complexity
    /// - Time: `O((n + m) log(n + m))` dominated by sorting, plus linear collection work
    /// - Extra space: `O(n + m)` for vectors and rebuilt nodes
    ///
    /// # Notes
    /// - Spawning a thread has overhead; for small lists this can be slower than the
    ///   single-threaded version.
    /// - This uses `JoinHandle::join().unwrap()`: a panic in the spawned thread will panic
    ///   here as well.
    pub fn merge_two_lists_parallel(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::thread::{JoinHandle, spawn};

        let v1: JoinHandle<Vec<i32>> = spawn(move || {
            let mut nodes: Vec<i32> = Vec::new();

            let mut node = list1.as_ref();
            while node.is_some() {
                nodes.push(node.unwrap().val);
                node = node.unwrap().next.as_ref();
            }

            nodes
        });

        let mut nodes: Vec<i32> = Vec::new();
        let mut node = list2.as_ref();
        while node.is_some() {
            nodes.push(node.unwrap().val);
            node = node.unwrap().next.as_ref();
        }

        let mut v1 = v1.join().unwrap();
        nodes.append(&mut v1);
        nodes.sort();

        nodes.into_iter().rev().fold(None, |node, val| {
            let prev_node = Box::new(ListNode { val, next: node });
            Some(prev_node)
        })
    }
}

#[cfg(test)]
mod merge_two_lists_test {
    use super::*;

    macro_rules! test_case {
        ($list1: expr, $list2: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let list1: Option<Box<ListNode>> = $list1;
                let list2: Option<Box<ListNode>> = $list2;
                let expected: Option<Box<ListNode>> = $expected;

                let result: Option<Box<ListNode>> = Solutions::merge_two_lists(list1, list2);
                assert_eq!(result, expected);

                let list1: Option<Box<ListNode>> = $list1;
                let list2: Option<Box<ListNode>> = $list2;
                let result: Option<Box<ListNode>> =
                    Solutions::merge_two_lists_parallel(list1, list2);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(
        ListNode::build_from_vec(vec![1, 2, 3, 4]),
        ListNode::build_from_vec(vec![1, 2, 3, 4]),
        ListNode::build_from_vec(vec![1, 1, 2, 2, 3, 3, 4, 4]),
        test_case_1
    );

    test_case!(None, None, None, test_case_2);

    test_case!(
        ListNode::build_from_vec(vec![2]),
        ListNode::build_from_vec(vec![1]),
        ListNode::build_from_vec(vec![1, 2]),
        test_case_3
    );

    test_case!(
        ListNode::build_from_vec(vec![5, 6, 7, 8]),
        ListNode::build_from_vec(vec![1, 2, 3, 4]),
        ListNode::build_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        test_case_4
    );

    test_case!(
        ListNode::build_from_vec(vec![1, 1, 1, 1]),
        ListNode::build_from_vec(vec![1, 1, 1, 1]),
        ListNode::build_from_vec(vec![1, 1, 1, 1, 1, 1, 1, 1]),
        test_case_5
    );

    test_case!(
        ListNode::build_from_vec(vec![9, 0, 9, 0, 8, 9]),
        ListNode::build_from_vec(vec![4, 3, 4, 3, 2, 3]),
        ListNode::build_from_vec(vec![0, 0, 2, 3, 3, 3, 4, 4, 8, 9, 9, 9]),
        test_case_6
    );
}
