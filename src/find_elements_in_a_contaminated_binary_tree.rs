//! Solution for https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree
//! 1261. Find Elements in a Contaminated Binary Tree

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
use std::collections::BTreeSet;
use std::rc::Rc;

#[derive(Default)]
pub struct FindElements(BTreeSet<i32>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn recurse(&mut self, root: Option<Rc<RefCell<TreeNode>>>, index: i32) {
        if let Some(root) = root {
            self.0.insert(index);
            let mut root = root.borrow_mut();
            self.recurse(root.left.take(), 2 * index + 1);
            self.recurse(root.right.take(), 2 * index + 2);
        }
    }

    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut new = Self::default();
        new.recurse(root, 0);
        new
    }

    pub fn find(&self, target: i32) -> bool {
        self.0.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

pub struct Solution;
use cargo_leet::TreeNode;
// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    #[test]
    fn leet_1() {
        let root = TreeRoot::from("[-1,null,-1]").into();
        let find_elements = FindElements::new(root);
        assert!(!find_elements.find(1));
        assert!(find_elements.find(2));
    }

    #[test]
    fn leet_2() {
        let root = TreeRoot::from("[-1,-1,-1,-1,-1]").into();
        let find_elements = FindElements::new(root);
        assert!(find_elements.find(1));
        assert!(find_elements.find(3));
        assert!(!find_elements.find(5));
    }

    #[test]
    fn leet_3() {
        let root = TreeRoot::from("[-1,null,-1,-1,null,-1]").into();
        let find_elements = FindElements::new(root);
        assert!(find_elements.find(2));
        assert!(!find_elements.find(3));
        assert!(!find_elements.find(4));
        assert!(find_elements.find(5));
    }
}
