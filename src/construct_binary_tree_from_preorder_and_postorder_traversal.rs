//! Solution for https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal
//! 889. Construct Binary Tree from Preorder and Postorder Traversal

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
    pub fn construct_from_pre_post_(
        current: i32,
        preorder: &[i32],
        postorder: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = TreeNode::new(current);
        if let Some((&left, preorder)) = preorder.split_first() {
            let (&right, postorder) = postorder.split_last().unwrap();
            if right == left {
                node.left = Self::construct_from_pre_post_(left, preorder, postorder);
            } else {
                let left_pos = postorder.iter().position(|i| *i == left).unwrap();
                let (left_postorder, right_postorder) = postorder.split_at(left_pos);
                let right_pos = preorder.iter().position(|i| *i == right).unwrap();
                let (left_preorder, right_preorder) = preorder.split_at(right_pos);
                node.left = Self::construct_from_pre_post_(left, left_preorder, left_postorder);
                node.right = Self::construct_from_pre_post_(
                    right,
                    &right_preorder[1..],
                    &right_postorder[1..],
                );
            }
        }
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (_, preorder) = preorder.split_first().unwrap();
        let (&n, postorder) = postorder.split_last().unwrap();
        Self::construct_from_pre_post_(n, &preorder, &postorder)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>
use cargo_leet::TreeNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use cargo_leet::TreeRoot;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4,5,3,6,7], vec![4,5,2,6,7,3,1], TreeRoot::from("[1,2,3,4,5,6,7]").into())]
    #[case(vec![1], vec![1], TreeRoot::from("[1]").into())]
    fn case(
        #[case] preorder: Vec<i32>,
        #[case] postorder: Vec<i32>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        let actual = Solution::construct_from_pre_post(preorder, postorder);
        assert_eq!(actual, expected);
    }
}
