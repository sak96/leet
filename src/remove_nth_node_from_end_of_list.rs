//! Solution for https://leetcode.com/problems/remove-nth-node-from-end-of-list
//! 19. Remove Nth Node From End of List

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
    fn len_in_reverse(head: &mut Option<Box<ListNode>>, n: i32) -> i32 {
        match head {
            Some(ref mut node) => {
                let length = Self::len_in_reverse(&mut node.next, n) + 1;
                if length == n {
                    let next = node.next.take();
                    *head = next;
                }
                length
            }
            None => 0,
        }
    }

    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::len_in_reverse(&mut head, n);
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, ListHead::from(vec![1,2,3,5]).into())]
    #[case(ListHead::from(vec![1]).into(), 1, ListHead::from(vec![]).into())]
    #[case(ListHead::from(vec![1,2]).into(), 1, ListHead::from(vec![1]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] n: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::remove_nth_from_end(head, n);
        assert_eq!(actual, expected);
    }
}
