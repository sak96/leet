use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn bst_from_preorder_slice(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        // get first element and rest of array
        if let Some((val, preorder)) = preorder.split_first() {
            // all small elements go to left, others to right
            let pos = preorder.partition_point(|x| x < val);
            let (left, right) = preorder.split_at(pos);

            // create tree node from first element
            Some(Rc::new(RefCell::new(TreeNode {
                val: *val,
                left: Self::bst_from_preorder_slice(left),
                right: Self::bst_from_preorder_slice(right),
            })))
        } else {
            None
        }
    }
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst_from_preorder_slice(&preorder)
    }
}

use crate::helpers::tree_nodes::TreeNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let preorder = [8, 5, 1, 7, 10, 12];
        let output = TreeNode::create("[8,5,10,1,7,null,12]");
        assert_eq!(Solution::bst_from_preorder(preorder.into()), output);
    }

    #[test]
    fn case2() {
        let preorder = [1, 3];
        let output = TreeNode::create("[1,null ,3 ]");
        assert_eq!(Solution::bst_from_preorder(preorder.into()), output);
    }
}
