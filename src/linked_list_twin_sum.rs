impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut storage = std::collections::VecDeque::with_capacity(100_000);
        while let Some(node) = head.take() {
            storage.push_front(node.val);
            head = node.next;
        }

        let mut max = 0;
        while !storage.is_empty() {
            let a = storage.pop_front().unwrap();
            let b = storage.pop_back().unwrap();
            max = std::cmp::max(max, a + b);
        }

        max
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
        let head = ListNode::create_list([5, 4, 2, 1].into());
        assert_eq!(Solution::pair_sum(head), 6);
    }

    #[test]
    fn case2() {
        let head = ListNode::create_list([4, 2, 2, 3].into());
        assert_eq!(Solution::pair_sum(head), 7);
    }

    #[test]
    fn case3() {
        let head = ListNode::create_list([1, 100000].into());
        assert_eq!(Solution::pair_sum(head), 100001);
    }
}
