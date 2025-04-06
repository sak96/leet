//! Solution for https://leetcode.com/problems/binary-tree-inorder-traversal
//! 94. Binary Tree Inorder Traversal

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
    pub fn inorder_traversal_(root: &Option<Rc<RefCell<TreeNode>>>, output: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder_traversal_(&node.left, output);
            output.push(node.val);
            Self::inorder_traversal_(&node.right, output);
        }
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output = vec![];
        Self::inorder_traversal_(&root, &mut output);
        output
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
    #[case(TreeRoot::from("[1,null,2,3]").into(), todo!("Expected Result"))]
    #[case(TreeRoot::from("[1,2,3,4,5,null,8,null,null,6,7,9]").into(), todo!("Expected Result"))]
    #[case(TreeRoot::from("[]").into(), todo!("Expected Result"))]
    #[case(TreeRoot::from("[1]").into(), todo!("Expected Result"))]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<i32>) {
        let actual = Solution::inorder_traversal(root);
        assert_eq!(actual, expected);
    }
}
