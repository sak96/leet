impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        let mut first_k_node = head.as_mut().unwrap() as *mut Box<ListNode>;
        // get first kth node
        while k > 0 {
            unsafe {
                first_k_node = (*first_k_node).next.as_mut().unwrap();
            }
            k -= 1;
        }

        // get second kth node
        let mut end_node = first_k_node;
        let mut second_k_node = head.as_mut().unwrap() as *mut Box<ListNode>;
        unsafe {
            while (*end_node).next.is_some() {
                end_node = (*end_node).next.as_mut().unwrap();
                second_k_node = (*second_k_node).next.as_mut().unwrap();
            }
            // swap the node values
            std::mem::swap(&mut (*first_k_node).val, &mut (*second_k_node).val);
        }

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
