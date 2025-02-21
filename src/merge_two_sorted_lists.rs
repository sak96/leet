//! Solution for https://leetcode.com/problems/merge-two-sorted-lists
//! 21. Merge Two Sorted Lists

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(node)) | (Some(node), None) => Some(node),
            (Some(mut n1), Some(mut n2)) => {
                if n2.val < n1.val {
                    std::mem::swap(&mut n1, &mut n2);
                }
                n1.next = Self::merge_two_lists(n1.next.take(), Some(n2));
                Some(n1)
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
    #[case(ListHead::from(vec![1,2,4]).into(), ListHead::from(vec![1,3,4]).into(),ListHead::from(vec![1,1,2,3,4,4]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![]).into(), ListHead::from(vec![]).into())]
    #[case(ListHead::from(vec![]).into(), ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into())]
    fn case(
        #[case] list1: Option<Box<ListNode>>,
        #[case] list2: Option<Box<ListNode>>,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::merge_two_lists(list1, list2);
        assert_eq!(actual, expected);
    }
}
