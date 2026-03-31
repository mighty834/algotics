// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn build_from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        if v.len() == 0 {
            return None;
        };

        v.iter().rev().fold(None, |node, &v_val| {
            let mut prev_node = Box::new(ListNode::new(v_val));
            prev_node.next = node;
            Some(prev_node)
        })
    }
}
