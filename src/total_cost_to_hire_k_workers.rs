//! Solution for https://leetcode.com/problems/total-cost-to-hire-k-workers
//! 2462. Total Cost to Hire K Workers

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut total_cost = 0i64;
        let mut costs = std::collections::VecDeque::from(costs);
        let mut heap = std::collections::BinaryHeap::with_capacity(candidates as usize * 2);
        for _ in 0..candidates {
            if let Some(cost) = costs.pop_front() {
                heap.push((-cost, true));
            }
            if let Some(cost) = costs.pop_back() {
                heap.push((-cost, false));
            }
        }
        for _ in 0..k {
            let (cost, first_session) = heap.pop().unwrap();
            total_cost -= cost as i64;
            let next_candidate = if first_session {
                costs.pop_front()
            } else {
                costs.pop_back()
            };
            if let Some(value) = next_candidate {
                heap.push((-value, first_session));
            }
        }
        total_cost
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![17,12,10,2,7,2,11,20,8], 3, 4, 11)]
    #[case(vec![1,2,4,1], 3, 3, 4)]
    fn case(
        #[case] costs: Vec<i32>,
        #[case] k: i32,
        #[case] candidates: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::total_cost(costs, k, candidates);
        assert_eq!(actual, expected);
    }
}
