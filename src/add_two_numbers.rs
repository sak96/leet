//! Solution for https://leetcode.com/problems/add-two-numbers
//! 2. Add Two Numbers

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
    fn _add_two_numbers(
        l1: &mut Option<Box<ListNode>>,
        l2: &mut Option<Box<ListNode>>,
        mut carry: i32,
    ) {
        match (l1.as_mut(), l2.as_mut(), carry) {
            (Some(x), Some(y), _) => {
                let z = x.val + y.val + carry;
                carry = z / 10;
                x.val = z % 10;
                Self::_add_two_numbers(&mut x.next, &mut y.next, carry);
            }
            (None, Some(_), _) => {
                std::mem::swap(l1, l2);
                Self::_add_two_numbers(l1, l2, carry);
            }
            (_, None, mut carry) if carry > 0 => {
                let mut l = l1;
                while carry > 0 {
                    if let Some(x) = l {
                        let z = x.val + carry;
                        x.val = z % 10;
                        carry = z / 10;
                        l = &mut x.next;
                    } else {
                        *l = Some(Box::new(ListNode::new(carry)));
                        carry = 0;
                    }
                }
            }
            _ => {}
        }
    }
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::_add_two_numbers(&mut l1, &mut l2, 0);
        l1
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
    #[case(ListHead::from(vec![2,4,3]).into(), ListHead::from(vec![5,6,4]).into(), ListHead::from(vec![7,0,8]).into())]
    #[case(ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into(), ListHead::from(vec![0]).into())]
    #[case(ListHead::from(vec![9,9,9,9,9,9,9]).into(), ListHead::from(vec![9,9,9,9]).into(), ListHead::from(vec![8,9,9,9,0,0,0,1]).into())]
    fn case(
        #[case] l1: Option<Box<ListNode>>,
        #[case] l2: Option<Box<ListNode>>,
        #[case] expected: Option<Box<ListNode>>,
    ) {
        let actual = Solution::add_two_numbers(l1, l2);
        assert_eq!(actual, expected);
    }
}
