//! Solution for https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid
//! 2033. Minimum Operations to Make a Uni-Value Grid

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut input = Vec::with_capacity(grid.len() * grid[0].len());
        let modulo = grid[0][0] % x;
        for r in grid {
            for c in r {
                if c % x == modulo {
                    input.push(c - modulo);
                } else {
                    return -1;
                }
            }
        }
        input.sort_unstable();
        let mut queue = std::collections::VecDeque::from(input);
        let (mut count, mut ops) = (1, 0);
        while !queue.is_empty() {
            if let Some(d1) = queue.pop_front() {
                if let Some(d2) = queue.front() {
                    ops += ((d2 - d1) / x) * count;
                }
            }
            if let Some(d1) = queue.pop_back() {
                if let Some(d2) = queue.back() {
                    ops += ((d1 - d2) / x) * count;
                }
            }
            count += 1;
        }

        ops
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,4],vec![6,8]], 2, 4)]
    #[case(vec![vec![1,5],vec![2,3]], 1, 5)]
    #[case(vec![vec![1,2],vec![3,4]], 2, -1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] x: i32, #[case] expected: i32) {
        let actual = Solution::min_operations(grid, x);
        assert_eq!(actual, expected);
    }
}
