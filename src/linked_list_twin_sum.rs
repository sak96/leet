impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut half_ptr = head.as_mut().unwrap() as *mut Box<ListNode>;
        let mut full_ptr = head.as_mut().unwrap().next.as_mut().unwrap() as *mut Box<ListNode>;
        #[allow(unused_assignments)]
        let mut second_half = None;

        // travel to half
        unsafe {
            while (*full_ptr).next.is_some() {
                half_ptr = (*half_ptr).next.as_mut().unwrap() as *mut Box<ListNode>;
                full_ptr =
                    (*full_ptr).next.as_mut().unwrap().next.as_mut().unwrap() as *mut Box<ListNode>;
            }
            second_half = (*half_ptr).next.take();
        }

        // reverse second half;
        let mut reversed_second_half = None;
        while let Some(mut node) = second_half.take() {
            second_half = node.next.take();
            node.next = reversed_second_half;
            reversed_second_half = Some(node);
        }

        // get max sum of pairs.
        let mut max = 0;
        while let (Some(mut start), Some(mut end)) = (head, reversed_second_half) {
            max = std::cmp::max(max, start.val + end.val);
            head = start.next.take();
            reversed_second_half = end.next.take();
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
