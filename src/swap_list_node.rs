impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // find length
        let mut len = 0;
        let mut cur_ptr = head.as_ref();
        while let Some(node) = cur_ptr {
            len += 1;
            cur_ptr = node.next.as_ref();
        }

        // no swap if longer
        // k <= n hence below not applicable
        // if len <= k {
        //     return head;
        // }

        // get index of swapping
        use std::cmp::Ordering;
        let i = k - 1;
        let j = len - k;
        let (mut low, mut high) = match i.cmp(&j) {
            Ordering::Less => (i, j),
            Ordering::Greater => (j, i),
            _ => return head,
        };

        // get first node
        let mut left_part = &mut head;
        while low > 0 {
            left_part = &mut left_part.as_mut().unwrap().next;
            low -= 1;
            high -= 1;
        }

        let mut right_part = left_part.as_mut().unwrap().next.take();
        high -= 1;

        // get node two
        {
            let mut right_part = &mut right_part;
            while high > 0 {
                right_part = &mut right_part.as_mut().unwrap().next;
                high -= 1;
            }

            // swap the values
            std::mem::swap(
                &mut left_part.as_mut().unwrap().val,
                &mut right_part.as_mut().unwrap().val,
            );
        }

        // assign right_part back
        left_part.as_mut().unwrap().next = right_part;

        // return
        head
    }
}
pub struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn create_list(input: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in input.into_iter().rev() {
            head = Some(Box::new(ListNode { next: head, val }));
        }
        head
    }

    pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut output = vec![];
        while let Some(node) = head {
            output.push(node.val);
            head = node.next;
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let head = ListNode::create_list([1, 2, 3, 4, 5].into());
        let output = ListNode::to_vec(Solution::swap_nodes(head, 2));
        assert_eq!(output, [1, 4, 3, 2, 5]);
    }

    #[test]
    fn case2() {
        let input = [7, 9, 6, 6, 7, 8, 3, 0, 9, 5];
        let head = ListNode::create_list(input.into());
        let output = ListNode::to_vec(Solution::swap_nodes(head, 5));
        assert_eq!(output, [7, 9, 6, 6, 8, 7, 3, 0, 9, 5], "{:?}", input);
    }
}
