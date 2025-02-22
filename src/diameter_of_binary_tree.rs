//! Solution for https://leetcode.com/problems/diameter-of-binary-tree
//! 543. Diameter of Binary Tree

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn diameter_and_depth(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let (d1, l1) = Self::diameter_and_depth(node.borrow_mut().left.take());
            let (d2, l2) = Self::diameter_and_depth(node.borrow_mut().right.take());
            (d1.max(d2).max(l1 + l2 + 1), l1.max(l2) + 1)
        } else {
            (0, 0)
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter_and_depth(root).0 - 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,2,3,4,5]").into(), 3)]
    #[case(TreeRoot::from("[1,2]").into(), 1)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::diameter_of_binary_tree(root);
        assert_eq!(actual, expected);
    }
}
