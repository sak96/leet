impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut node = head.take().expect("minimum 2 nodes expected");
        head = node.next.take();
        let mut reversed_half = Some(node);
        let mut ptr = head.as_mut().expect("minimum 2 nodes expected") as *mut Box<ListNode>;

        // reverse first half
        // first half reached when ptr.next is none
        while unsafe { (*ptr).next.is_some() } {
            unsafe {
                ptr = (*ptr).next.as_mut().unwrap().next.as_mut().unwrap() as *mut Box<ListNode>;
            }
            let mut node = head.take().unwrap();
            head = node.next.take();
            node.next = reversed_half;
            reversed_half = Some(node);
        }

        // get max sum of pairs.
        let mut max = 0;
        while let (Some(mut start), Some(mut end)) = (head, reversed_half) {
            max = std::cmp::max(max, start.val + end.val);
            head = start.next.take();
            reversed_half = end.next.take();
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
