//! Solution for https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii
//! 1751. Maximum Number of Events That Can Be Attended II

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = events.len();
        let k = k as usize;
        events.sort_unstable();
        let mut memory = vec![vec![0; n + 1]; k + 1];
        (0..n).rev().for_each(|considered_events| {
            let next_events =
                events.partition_point(|event| event[0] <= events[considered_events][1]);
            (1..(k + 1)).for_each(|k| {
                memory[k][considered_events] = std::cmp::max(
                    memory[k][considered_events + 1],
                    events[considered_events][2] + memory[k - 1][next_events],
                );
            });
        });
        memory[k][0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,4],vec![3,4,3],vec![2,3,1]], 2, 7)]
    #[case(vec![vec![1,2,4],vec![3,4,3],vec![2,3,10]], 2, 10)]
    #[case(vec![vec![1,1,1],vec![2,2,2],vec![3,3,3],vec![4,4,4]], 3, 9)]
    fn case(#[case] events: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_value(events, k);
        assert_eq!(actual, expected);
    }
}
