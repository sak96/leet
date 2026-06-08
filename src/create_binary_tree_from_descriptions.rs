//! Solution for https://leetcode.com/problems/create-binary-tree-from-descriptions
//! 2196. Create Binary Tree From Descriptions

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(
        map: &mut std::collections::HashMap<i32, (Option<i32>, Option<i32>)>,
        index: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = TreeNode::new(index);
        if let Some((left, right)) = map.remove(&index) {
            if let Some(left) = left {
                node.left = Self::build_tree(map, left);
            }
            if let Some(right) = right {
                node.right = Self::build_tree(map, right);
            }
        }
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if descriptions.is_empty() {
            return None;
        }
        let mut map = std::collections::HashMap::new();
        let mut children = std::collections::HashSet::new();
        for d in descriptions {
            let entry = map.entry(d[0]).or_insert((None, None));
            if d[2] == 0 {
                entry.1 = Some(d[1]);
            } else {
                entry.0 = Some(d[1]);
            }
            children.insert(d[1]);
        }
        let root = map
            .keys()
            .find(|k| !children.contains(*k))
            .cloned()
            .expect("root is required");
        Self::build_tree(&mut map, root)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

use cargo_leet::TreeNode;
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use cargo_leet::TreeRoot;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![20,15,1],vec![20,17,0],vec![50,20,1],vec![50,80,0],vec![80,19,1]], TreeRoot::from("[50,20,80,15,17,19]").into())]
    #[case(vec![vec![1,2,1],vec![2,3,0],vec![3,4,1]], TreeRoot::from("[1,2,null,null,3,4]").into())]
    fn case(#[case] descriptions: Vec<Vec<i32>>, #[case] expected: Option<Rc<RefCell<TreeNode>>>) {
        let actual = Solution::create_binary_tree(descriptions);
        assert_eq!(actual, expected);
    }
}
