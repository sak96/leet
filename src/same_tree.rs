//! Solution for https://leetcode.com/problems/same-tree
use std::cell::RefCell;
use std::rc::Rc;

use crate::helpers::tree_nodes::TreeNode;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            true
        } else if let (Some(p), Some(q)) = dbg!(p, q) {
            let mut p = p.borrow_mut();
            let mut q = q.borrow_mut();
            p.val == q.val
                && (Self::is_same_tree(p.left.take(), q.left.take()))
                && (Self::is_same_tree(p.right.take(), q.right.take()))
        } else {
            false
        }
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1("[1,2,3]", "[1,2,3]", true)]
    #[case::leet1("[1,2]", "[1,null,2]", false)]
    #[case::leet1("[1,2,1]", "[1,1,2]", false)]
    fn test(#[case] p: &str, #[case] q: &str, #[case] equal: bool) {
        let p = TreeNode::create(p);
        let q = TreeNode::create(q);
        assert_eq!(Solution::is_same_tree(p, q), equal);
    }
}
