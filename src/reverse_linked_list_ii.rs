//! Solution for https://leetcode.com/problems/reverse-linked-list-ii
//! 92. Reverse Linked List II

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut left_ptr = &mut head;
        for _ in 1..left {
            left_ptr = &mut left_ptr.as_mut().expect("left <= n").next;
        }
        let mut left_node = left_ptr.take();
        let mut right_ptr = left_node.as_mut().expect("left != right && right <=n");
        for _ in left..right {
            right_ptr = right_ptr.next.as_mut().expect("right <= n");
        }
        let mut right_node = right_ptr.next.take();
        while let Some(mut node) = left_node.take() {
            left_node = node.next.take();
            node.next = right_node;
            right_node = Some(node);
        }
        *left_ptr = right_node;
        head
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::ListNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::ListHead;

    use rstest::rstest;

    #[rstest]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, 4, ListHead::from(vec![1,4,3,2,5]).into())]
    #[case(ListHead::from(vec![5]).into(), 1, 1, ListHead::from(vec![5]).into())]
    #[case(ListHead::from(vec![3, 5]).into(), 1, 2, ListHead::from(vec![5, 3]).into())]
    #[case(ListHead::from(vec![1,2,3,4,2,5]).into(), 2, 2, ListHead::from(vec![1,2,3,4,2,5]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] left: i32,
        #[case] right: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::reverse_between(head, left, right);
        assert_eq!(actual, expected);
    }
}
