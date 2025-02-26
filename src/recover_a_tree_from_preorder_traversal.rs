//! Solution for https://leetcode.com/problems/recover-a-tree-from-preorder-traversal
//! 1028. Recover a Tree From Preorder Traversal

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
use std::iter::Peekable;
use std::rc::Rc;
impl Solution {
    fn generate_tree<I>(depth: i32, traversal: &mut Peekable<I>) -> Option<Rc<RefCell<TreeNode>>>
    where
        I: Iterator<Item = (i32, i32)>,
    {
        if let Some((_, num)) = traversal.next_if(|&(d, _)| d == depth) {
            let mut node = TreeNode::new(num);
            node.left = Self::generate_tree(depth + 1, traversal);
            node.right = Self::generate_tree(depth + 1, traversal);
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }

    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut chars = traversal.chars().peekable();
        let mut traversal = vec![];
        while chars.peek().is_some() {
            let mut depth = 0;
            while chars.next_if_eq(&'-').is_some() {
                depth += 1;
            }
            let mut num = 0i32;
            while let Some(digit) = chars.next_if(|&c| c.is_ascii_digit()) {
                num *= 10;
                num += digit.to_digit(10).unwrap() as i32;
            }
            traversal.push((depth, num));
        }
        Self::generate_tree(0, &mut traversal.into_iter().peekable())
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
    #[case("1-2--3--4-5--6--7", TreeRoot::from("[1,2,5,3,4,6,7]").into())]
    #[case("1-2--3---4-5--6---7", TreeRoot::from("[1,2,5,3,null,6,null,4,null,7]").into())]
    #[case("1-401--349---90--88", TreeRoot::from("[1,401,null,349,88,90]").into())]
    fn case(#[case] traversal: String, #[case] expected: Option<Rc<RefCell<TreeNode>>>) {
        let actual = Solution::recover_from_preorder(traversal);
        assert_eq!(actual, expected);
    }
}
