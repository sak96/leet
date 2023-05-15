impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let mut fast = &head;
        let mut slow = head.as_ref().expect("Constraint is n >= 1");

        // Move fast ahead of slow by k
        for _ in 0..k {
            fast = &fast
                .as_ref()
                .expect("Based on constraint in question k <= n")
                .next;
        }

        // Move slow to k before the end
        let mut second_index = 0; // TODO switch to 1 based indexing
        while let Some(node) = fast {
            fast = &node.next;
            slow = slow
                .next
                .as_ref()
                .expect("This is behind fast so must exist");
            second_index += 1;
        }

        // Save value from 2nd node to update the first
        let second_value = slow.val;

        // Walk the list again and update the nodes
        let mut node = &mut head;
        for _ in 0..k - 1 {
            node = &mut node.as_mut().expect("k <= n by constraint").next;
        }
        let first_value = node.as_ref().expect("At first node").val;
        node.as_mut().expect("At first node").val = second_value;
        for _ in k - 1..second_index {
            node = &mut node.as_mut().expect("k <= n by constraint").next;
        }
        node.as_mut().expect("At second node").val = first_value;

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
