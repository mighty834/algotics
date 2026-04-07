use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }

    pub fn build_from_vec(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if v.is_empty() || v[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(v[0]?)));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;

        while i < v.len() {
            let parent = match queue.pop_front() {
                Some(node) => node,
                None => break,
            };

            if i < v.len() {
                if let Some(val) = v[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    parent.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }

            if i < v.len() {
                if let Some(val) = v[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    parent.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }

    pub fn build_from_int_vec(v: Vec<i32>, none_type_int: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let vec: Vec<Option<i32>> =
            v.into_iter().map(|val| if val == none_type_int { None } else { Some(val) }).collect();

        Self::build_from_vec(vec)
    }
}
