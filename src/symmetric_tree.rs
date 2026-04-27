//! Solution for https://leetcode.com/problems/symmetric-tree
//! 101. Symmetric Tree

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
    pub fn _is_symmetric(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                left.val == right.val
                    && Self::_is_symmetric(&left.left, &right.right)
                    && Self::_is_symmetric(&left.right, &right.left)
            }
            _ => false,
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => {
                let root = root.borrow();
                Self::_is_symmetric(&root.left, &root.right)
            }
            None => true,
        }
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
    #[case(TreeRoot::from("[1,2,2,3,4,4,3]").into(), true)]
    #[case(TreeRoot::from("[1,2,2,null,3,null,3]").into(), false)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: bool) {
        let actual = Solution::is_symmetric(root);
        assert_eq!(actual, expected);
    }
}
