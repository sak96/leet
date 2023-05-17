use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn create(input: &str) -> Option<Rc<RefCell<TreeNode>>> {
        // make sure end and start are brackets
        assert!(input.len() > 2, "should contain at least []");
        assert_eq!('[', input.chars().next().unwrap());
        assert_eq!(']', input.chars().nth(input.len() - 1).unwrap());
        let input = &input[1..input.len() - 1];

        // generate node from content inside brackets
        let nodes: Vec<_> = input
            .split(',')
            .map(|node| {
                // parse each element
                node.trim().ne("null").then(|| {
                    Rc::new(RefCell::new(TreeNode::new(
                        node.trim().parse::<i32>().expect("failed to parse"),
                    )))
                })
            })
            .collect();

        let mut child = 1;
        let len = nodes.len();

        // # order of elements
        // first node,
        // left + right child of first node,
        // left + right child of second node,
        // ....
        // NOTE: children of null are skipped
        // a.k.a. null terminated level order representation
        // ref: https://support.leetcode.com/hc/en-us/articles/360011883654-What-does-1-null-2-3-mean-in-binary-tree-representation-
        //
        // assign child to parents
        for parent in nodes.iter().flatten() {
            if child >= len {
                break; // no more child assignment
            }
            if let Some(Some(left)) = nodes.get(child) {
                parent.borrow_mut().left = Some(Rc::clone(left));
            }
            if let Some(Some(right)) = nodes.get(child + 1) {
                parent.borrow_mut().right = Some(Rc::clone(right));
            }
            child += 2;
        }

        // send the root node
        nodes.into_iter().next().flatten()
    }
}
