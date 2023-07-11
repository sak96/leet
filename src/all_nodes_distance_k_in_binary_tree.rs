use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn distance_k_(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        k: i32,
        output: &mut Vec<i32>,
    ) -> i32 {
        if let Some(root) = root {
            let root_brw = root.borrow();
            let (mut node, mut depth, mut height) = (None, -1, -1);
            if root_brw.val == val {
                node = Some(root.clone());
                depth = k;
                height = k;
            } else {
                let left = Self::distance_k_(root_brw.left.clone(), val, k, output);
                if left >= 0 {
                    height = left;
                    depth = left - 1;
                    // target on left side so dfs on right side;
                    node = root_brw.right.clone();
                } else {
                    let right = Self::distance_k_(root_brw.right.clone(), val, k, output);
                    if right >= 0 {
                        height = right;
                        depth = height - 1;
                        node = root_brw.left.clone();
                    }
                }
            }
            if height == 0 {
                output.push(root_brw.val);
            } else if height >= 0 {
                Self::dfs(node, depth, output);
                return height - 1;
            }
        }
        -1
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, k: i32, output: &mut Vec<i32>) {
        debug_assert!(k >= 0);
        if let Some(root) = root {
            let root = root.borrow();
            if k == 0 {
                output.push(root.val);
            } else {
                Self::dfs(root.left.clone(), k - 1, output);
                Self::dfs(root.right.clone(), k - 1, output);
            }
        }
    }

    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let val = target.unwrap().borrow().val;
        let mut output = vec![];
        Self::distance_k_(root, val, k, &mut output);
        output
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
    #[case(TreeRoot::from("[3,5,1,6,2,0,8,null,null,7,4]").into(),5,2,vec![7,4,1])]
    fn case(
        #[case] root: Option<Rc<RefCell<TreeNode>>>,
        #[case] target: i32,
        #[case] k: i32,
        #[case] mut expected: Vec<i32>,
    ) {
        let target = Some(Rc::new(RefCell::new(TreeNode::new(target))));
        let mut output = Solution::distance_k(root, target, k);
        output.sort_unstable();
        expected.sort_unstable();
        assert_eq!(output, expected);
    }
}
