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
        assert!(input.len() > 2, "should contain at least []");
        assert_eq!('[', input.chars().next().unwrap());
        assert_eq!(']', input.chars().nth(input.len() - 1).unwrap());
        let input = &input[1..input.len() - 1];
        let nodes: Vec<_> = input
            .split(',')
            .map(|node| {
                node.trim().ne("null").then(|| {
                    Rc::new(RefCell::new(TreeNode::new(
                        node.trim().parse::<i32>().expect("failed to parse"),
                    )))
                })
            })
            .collect();

        let mut child = 1;
        let len = nodes.len();
        for node in nodes.iter().flatten() {
            if child >= len {
                break;
            }
            if let Some(Some(left)) = nodes.get(child) {
                node.borrow_mut().left = Some(Rc::clone(left));
            }
            if let Some(Some(right)) = nodes.get(child + 1) {
                node.borrow_mut().right = Some(Rc::clone(right));
            }
            child += 2;
        }

        nodes.into_iter().next().flatten()
    }
}
