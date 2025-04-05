//! Solution for https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves
//! 1123. Lowest Common Ancestor of Deepest Leaves

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn _lca_deepest_leaves(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
    ) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        match root {
            Some(n) => {
                let node = n.borrow();
                let (left, ldepth) = Self::_lca_deepest_leaves(&node.left, depth + 1);
                let (right, rdepth) = Self::_lca_deepest_leaves(&node.right, depth + 1);
                match ldepth.cmp(&rdepth) {
                    std::cmp::Ordering::Equal => (root.clone(), ldepth),
                    std::cmp::Ordering::Greater => (left, ldepth),
                    std::cmp::Ordering::Less => (right, rdepth),
                }
            }
            None => (None, depth + 1),
        }
    }
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_lca_deepest_leaves(&root, 0).0
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
    #[case(TreeRoot::from("[3,5,1,6,2,0,8,null,null,7,4]").into(), TreeRoot::from("[2,7,4]").into())]
    #[case(TreeRoot::from("[1]").into(), TreeRoot::from("[1]").into())]
    #[case(TreeRoot::from("[0,1,3,null,2]").into(), TreeRoot::from("[2]").into())]
    #[case(TreeRoot::from("[1,2,null,3,4,null,6,null,5]").into(), TreeRoot::from("[2,3,4,null,6,null,5]").into())]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] expected: Option<Rc<RefCell<TreeNode>>>,
    ) {
        let actual = Solution::lca_deepest_leaves(root);
        assert_eq!(actual, expected);
    }
}
