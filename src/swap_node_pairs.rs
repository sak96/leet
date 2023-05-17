impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node1 = if let Some(node) = head {
            node
        } else {
            return head;
        };
        let mut node2 = if let Some(node) = node1.next.take() {
            node
        } else {
            return Some(node1);
        };
        let rest = node2.next.take();
        node1.next = if rest.is_some() {
            Self::swap_pairs(rest)
        } else {
            rest
        };
        node2.next = Some(node1);
        Some(node2)
    }
}

pub struct Solution;
use crate::helpers::list_nodes::ListNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::create_list([1, 2, 3, 4].into());
        let output = ListNode::to_vec(Solution::swap_pairs(head));
        assert_eq!(output, [2, 1, 4, 3]);
    }

    #[test]
    fn case2() {
        let head = ListNode::create_list([1].into());
        let output = ListNode::to_vec(Solution::swap_pairs(head));
        assert_eq!(output, [1]);
    }

    #[test]
    fn case3() {
        let head = ListNode::create_list([].into());
        let output = ListNode::to_vec(Solution::swap_pairs(head));
        assert_eq!(output, []);
    }
}
