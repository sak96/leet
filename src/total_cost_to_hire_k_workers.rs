//! Solution for https://leetcode.com/problems/total-cost-to-hire-k-workers
//! 2462. Total Cost to Hire K Workers

impl Solution {
    pub fn total_cost(costs: Vec<i32>, mut k: i32, candidates: i32) -> i64 {
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
        while k > 0 && !costs.is_empty() {
            let (cost, first_session) = heap.pop().unwrap();
            total_cost -= cost as i64;
            k -= 1;
            let next_candidate = if first_session {
                costs.pop_front().unwrap()
            } else {
                costs.pop_back().unwrap()
            };
            heap.push((-next_candidate, first_session));
        }
        if k > 0 {
            let vec = heap.into_sorted_vec();
            total_cost -= vec
                .into_iter()
                .rev()
                .take(k as usize)
                .map(|x| x.0 as i64)
                .sum::<i64>()
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
    #[case(vec![28,35,21,13,21,72,35,52,74,92,25,65,77,1,73,32,43,68,8,100,84,80,14,88,42,53,98,69,64,40,60,23,99,83,5,21,76,34], 32, 12, 1407)]
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
