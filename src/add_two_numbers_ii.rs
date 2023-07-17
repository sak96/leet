//! Solution for https://leetcode.com/problems/add-two-numbers-ii
//! 445. Add Two Numbers II

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
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        while let Some(mut h) = head.take() {
            head = h.next.take();
            h.next = new_head.take();
            new_head = Some(h);
        }
        new_head
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = Self::reverse(l1);
        let mut l2 = Self::reverse(l2);
        let mut result = None;
        let mut borrow = 0;
        loop {
            match (l1.take(), l2.take()) {
                (Some(mut n1), Some(mut n2)) => {
                    l1 = n1.next.take();
                    l2 = n2.next.take();
                    n1.val += n2.val + borrow;
                    borrow = n1.val / 10;
                    n1.val %= 10;
                    n1.next = result.take();
                    result = Some(n1);
                }
                (None, Some(n1)) | (Some(n1), None) => {
                    l1 = Some(n1);
                    break;
                }
                _ => break,
            }
        }
        while let Some(mut n) = l1.take() {
            l1 = n.next.take();
            n.val += borrow;
            borrow = n.val / 10;
            n.val %= 10;
            n.next = result.take();
            result = Some(n);
        }
        if borrow != 0 {
            let mut node = Box::new(ListNode::new(borrow));
            node.next = result.take();
            result = Some(node);
        }
        result
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
    #[case(ListHead::from(vec![7,2,4,3]).into(), ListHead::from(vec![5,6,4]).into(), ListHead::from(vec![7,8,0,7]).into())]
    #[case(ListHead::from(vec![2,4,3]).into(), ListHead::from(vec![5,6,4]).into(), ListHead::from(vec![8,0,7]).into())]
    #[case(ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into())]
    fn case(
        #[case] l1: Option<Box<ListNode>>,
        #[case] l2: Option<Box<ListNode>>,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::add_two_numbers(l1, l2);
        assert_eq!(actual, expected);
    }
}
