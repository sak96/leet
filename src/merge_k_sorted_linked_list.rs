impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // NOTE: reverse to use min heap
        Some(self.val.cmp(&other.val).reverse())
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut ptr = &mut head;
        let mut heap: std::collections::BinaryHeap<_> = lists.into();
        while let Some(list) = heap.pop() {
            ptr.next = list;
            if let Some(ref mut node) = ptr.next {
                if let Some(rest) = node.next.take() {
                    heap.push(Some(rest))
                }
                ptr = node;
            }
        }
        head.next.take()
    }
}
pub struct Solution;
use crate::helpers::list_nodes::ListNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let lists: Vec<Vec<i32>> = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let output = [1, 1, 2, 3, 4, 4, 5, 6];
        let output = ListNode::create_list(output.into());
        let lists = lists
            .iter()
            .map(|x| ListNode::create_list(x.to_vec()))
            .collect();
        assert_eq!(Solution::merge_k_lists(lists), output);
    }

    #[test]
    fn case2() {
        let lists: Vec<Vec<i32>> = vec![];
        let output = [];
        let output = ListNode::create_list(output.into());
        let lists = lists
            .iter()
            .map(|x| ListNode::create_list(x.to_vec()))
            .collect();
        assert_eq!(Solution::merge_k_lists(lists), output);
    }

    #[test]
    fn case3() {
        let lists: Vec<Vec<i32>> = vec![vec![]];
        let output = [];
        let output = ListNode::create_list(output.into());
        let lists = lists
            .iter()
            .map(|x| ListNode::create_list(x.to_vec()))
            .collect();
        assert_eq!(Solution::merge_k_lists(lists), output);
    }
}
