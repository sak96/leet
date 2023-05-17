use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut root = root.borrow_mut();
            std::cmp::max(
                Solution::max_depth(root.left.take()),
                Solution::max_depth(root.right.take()),
            ) + 1
        } else {
            0
        }
    }
}
use crate::helpers::tree_nodes::TreeNode;

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let root = TreeNode::create("[3,9,20,null,null,15,7]");
        let output = 3;
        assert_eq!(Solution::max_depth(root), output);
    }

    #[test]
    fn case3() {
        let root = TreeNode::create("[1,null,2]");
        let output = 2;
        assert_eq!(Solution::max_depth(root), output);
    }
}
