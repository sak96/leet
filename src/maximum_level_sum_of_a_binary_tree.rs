//! Solution for https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level = 0;
        let mut max_level = -1;
        let mut max_value = i32::MIN;
        let mut queue = std::collections::VecDeque::new();
        queue.push_front(root);
        while !queue.is_empty() {
            level += 1;
            let mut value = 0;
            for _ in 0..queue.len() {
                if let Some(root) = queue.pop_back().flatten() {
                    value += root.borrow().val;
                    let left = root.borrow().left.clone();
                    let right = root.borrow().right.clone();
                    if left.is_some() {
                        queue.push_front(left.clone());
                    }
                    if right.is_some() {
                        queue.push_front(right.clone());
                    }
                }
            }
            dbg!(value, level);
            if value > max_value {
                max_value = value;
                max_level = level;
            }
        }
        max_level
    }
}

pub struct Solution;
use crate::helpers::tree_nodes::TreeNode;
#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    #[rstest]
    #[case::leet1(TreeNode::create("[1,7,0,7,-8,null,null]"), 2)]
    #[case::leet2(
        TreeNode::create("[989,null,10250,98693,-89388,null,null,null,-32127]"),
        2
    )]
    #[case::leet3(TreeNode::create("[-100,-200,-300,-20,-5,-10,null]"), 3)]
    fn test(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] output: i32) {
        assert_eq!(Solution::max_level_sum(root), output);
    }
}
