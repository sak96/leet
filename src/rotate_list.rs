//! Solution for https://leetcode.com/problems/rotate-list
//! 61. Rotate List

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
    pub fn rotate_right_(
        head: Option<Box<ListNode>>,
        k: i32,
        n: i32,
    ) -> (i32, Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if let Some(mut head) = head {
            let (k, head1, head2) = Self::rotate_right_(head.next.take(), k, n + 1);
            if k == 0 {
                head.next = head1;
                (k, Some(head), head2)
            } else {
                head.next = head2;
                (k - 1, head1, Some(head))
            }
        } else {
            ((k % n), None, None)
        }
    }

    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            head
        } else {
            let (_, head1, mut head2) = Self::rotate_right_(head, k, 0);
            if head2.is_some() {
                let mut ptr = &mut head2;
                while let Some(temp) = ptr {
                    if temp.next.is_none() {
                        temp.next = head1;
                        break;
                    }
                    ptr = &mut temp.next
                }
                head2
            } else {
                head1
            }
        }
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, ListHead::from(vec![4,5,1,2,3]).into())]
    #[case(ListHead::from(vec![0,1,2]).into(), 4, ListHead::from(vec![2,0,1]).into())]
    #[case(ListHead::from(vec![0,1,2]).into(), 3, ListHead::from(vec![0,1,2]).into())]
    #[case(ListHead::from(vec![0,1,2]).into(), 0, ListHead::from(vec![0,1,2]).into())]
    #[case(ListHead::from(vec![]).into(), 10, ListHead::from(vec![]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] k: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::rotate_right(head, k);
        assert_eq!(actual, expected);
    }
}
