//! Solution for https://leetcode.com/problems/split-linked-list-in-parts
//! 725. Split Linked List in Parts

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
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut len = 0;
        let mut iter_ptr = &head;
        while let Some(node) = iter_ptr {
            iter_ptr = &node.next;
            len += 1;
        }
        let mut output = Vec::with_capacity(k as usize);
        let size = len / k;
        let mut extra = len % k;
        for _ in 0..k {
            let part_size = size
                + if extra > 0 {
                    extra -= 1;
                    1
                } else {
                    0
                };
            let mut iter_mut = &mut head;
            for _ in 0..part_size {
                iter_mut = &mut iter_mut.as_mut().unwrap().next;
            }
            let new_head = iter_mut.take();
            output.push(head.take());
            head = new_head;
        }
        output
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
    #[case(ListHead::from(vec![1,2,3]).into(), 5, vec![ListHead::from(vec![1]).into(),ListHead::from(vec![2]).into(),ListHead::from(vec![3]).into(),ListHead::from(vec![]).into(),ListHead::from(vec![]).into()])]
    #[case(ListHead::from(vec![1,2,3,4,5,6,7,8,9,10]).into(), 3, vec![ListHead::from(vec![1,2,3,4]).into(),ListHead::from(vec![5,6,7]).into(),ListHead::from(vec![8,9,10]).into()])]
    fn case(
        #[case] head: Option<Box<ListNode>>,
        #[case] k: i32,
        #[case] expected: Vec<Option<Box<ListNode>>>,
    ) {
        let actual = Solution::split_list_to_parts(head, k);
        assert_eq!(actual, expected);
    }
}
