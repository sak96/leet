//! Solution for https://leetcode.com/problems/minimum-distance-between-bst-nodes
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>, prev_node: &mut Option<i32>) -> i32 {
        let mut min = i32::MAX;
        if let Some(root) = root {
            min = Self::inorder(root.borrow_mut().left.take(), prev_node);
            let val = root.borrow().val;
            if let Some(prev) = prev_node {
                min = min.min(val - *prev);
            }
            *prev_node = Some(val);
            min = min.min(Self::inorder(root.borrow_mut().right.take(), prev_node));
        }
        min
    }
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::inorder(root, &mut None)
    }
}

pub struct Solution;

use crate::helpers::tree_nodes::TreeNode;
#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    #[rstest]
    #[case::leet1(TreeNode::create("[4,2,6,1,3]"), 1)]
    #[case::leet1(TreeNode::create("[1,0,48,null,null,12,49]"), 1)]
    fn test(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] output: i32) {
        assert_eq!(Solution::min_diff_in_bst(root), output);
    }
}
