//! Solution for https://leetcode.com/problems/reverse-nodes-in-k-group
//! 25. Reverse Nodes in k-Group

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
    pub fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reverse_part = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = reverse_part;
            reverse_part = Some(node);
        }
        reverse_part
    }
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut reverse_part = None;
        let mut ptr = &mut reverse_part;
        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                *ptr = Some(node);
                ptr = &mut ptr.as_mut().unwrap().next;
            } else {
                return reverse_part;
            }
        }
        let mut reversed = Self::reverse(reverse_part);
        let mut ptr = &mut reversed;
        while let Some(node) = ptr {
            ptr = &mut node.next;
        }
        *ptr = Self::reverse_k_group(head, k);
        reversed
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
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 2, ListHead::from(vec![2,1,4,3,5]).into())]
    #[case(ListHead::from(vec![1,2,3,4,5]).into(), 3, ListHead::from(vec![3,2,1,4,5]).into())]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] k: i32,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::reverse_k_group(head, k);
        assert_eq!(actual, expected);
    }
}
