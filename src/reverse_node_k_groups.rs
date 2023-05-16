impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 1 means no swap
        if k == 1 {
            return head;
        }

        let mut ptr = &mut head;
        // get k - 1 th nodes
        for _ in 0..(k - 1) {
            if let Some(node) = ptr {
                ptr = &mut node.next;
            } else {
                return head;
            }
        }

        // make sure k th node is valid
        let rest = if let Some(ref mut node) = ptr {
            node.next.take()
        } else {
            return head;
        };

        // reverse the reset of node
        let mut new_head: Option<Box<ListNode>> = Self::reverse_k_group(rest, k);
        dbg!(&new_head);

        // attach the reverse nodes back to current
        while let Some(mut node1) = head.take() {
            head = node1.next.take();
            node1.next = new_head;
            new_head = Some(node1);
        }

        new_head
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
        let output = ListNode::to_vec(Solution::reverse_k_group(head, 2));
        assert_eq!(output, [2, 1, 4, 3, 5]);
    }

    #[test]
    fn case2() {
        let head = ListNode::create_list([1, 2, 3, 4, 5].into());
        let output = ListNode::to_vec(Solution::reverse_k_group(head, 3));
        assert_eq!(output, [3, 2, 1, 4, 5]);
    }

    #[test]
    fn case3() {
        let head = ListNode::create_list([].into());
        let output = ListNode::to_vec(Solution::reverse_k_group(head, 2));
        assert_eq!(output, []);
    }

    #[test]
    fn case4() {
        let head = ListNode::create_list([1].into());
        let output = ListNode::to_vec(Solution::reverse_k_group(head, 2));
        assert_eq!(output, [1]);
    }

    #[test]
    fn case5() {
        let head = ListNode::create_list([1, 2, 3, 4, 5].into());
        let output = ListNode::to_vec(Solution::reverse_k_group(head, 1));
        assert_eq!(output, [1, 2, 3, 4, 5]);
    }
}
