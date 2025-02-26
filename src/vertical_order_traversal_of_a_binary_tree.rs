//! Solution for https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree
//! 987. Vertical Order Traversal of a Binary Tree

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
    fn traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
        i: isize,
        j: isize,
        output: &mut Vec<(isize, isize, i32)>,
    ) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            dbg!(node.val, i, j);
            output.push((i, j, node.val));
            Self::traversal(node.left.take(), i - 1, j + 1, output);
            Self::traversal(node.right.take(), i + 1, j + 1, output);
        }
    }
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut output = vec![];
        Self::traversal(root, 0, 0, &mut output);
        output.sort();
        let mut result = vec![];
        let mut current = vec![];
        let mut idx = output.first().unwrap().0;
        for (i, _, v) in output.into_iter() {
            if i != idx {
                result.push(std::mem::take(&mut current));
                idx += 1;
            }
            current.push(v);
        }
        if !current.is_empty() {
            result.push(current);
        }
        result
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
    #[case(TreeRoot::from("[3,9,20,null,null,15,7]").into(), vec![vec![9],vec![3,15],vec![20],vec![7]])]
    //#[case(TreeRoot::from("[1,2,3,4,5,6,7]").into(), vec![vec![4],vec![2],vec![1,5,6],vec![3],vec![7]])]
    //#[case(TreeRoot::from("[1,2,3,4,6,5,7]").into(), vec![vec![4],vec![2],vec![1,5,6],vec![3],vec![7]])]
    //#[case(TreeRoot::from("[3,1,4,0,2,2]").into(), vec![vec![0],vec![1],vec![3,2,2],vec![4]])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::vertical_traversal(root);
        assert_eq!(actual, expected);
    }
}
