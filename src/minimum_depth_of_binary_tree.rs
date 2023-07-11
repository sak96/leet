//! Solution for https://leetcode.com/problems/minimum-depth-of-binary-tree
//! 111. Minimum Depth of Binary Tree

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_front(root);
            'outer: loop {
                depth += 1;
                let len = queue.len();
                for _ in 0..len {
                    let root = queue.pop_back().unwrap();
                    let root = root.borrow();
                    match (root.left.clone(), root.right.clone()) {
                        (None, None) => {
                            break 'outer;
                        }
                        (Some(left), None) => queue.push_front(left),
                        (None, Some(right)) => queue.push_front(right),
                        (Some(left), Some(right)) => {
                            queue.push_front(left);
                            queue.push_front(right);
                        }
                    }
                }
            }
        }
        depth
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
    #[case(TreeRoot::from("[3,9,20,null,null,15,7]").into(), 2)]
    #[case(TreeRoot::from("[2,null,3,null,4,null,5,null,6]").into(), 5)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::min_depth(root);
        assert_eq!(actual, expected);
    }
}
