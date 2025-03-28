//! Solution for https://leetcode.com/problems/maximum-number-of-points-from-grid-queries
//! 2503. Maximum Number of Points From Grid Queries

use std::cmp::Reverse;

impl Solution {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    pub fn max_points(mut grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut queries: Vec<_> = queries.iter().enumerate().map(|(i, q)| (q, i)).collect();
        queries.sort_unstable();
        let mut output = vec![0; queries.len()];
        let mut count = 0;
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Reverse((grid[0][0], 0, 0)));
        grid[0][0] = 0;
        for (query, index) in queries {
            while let Some(Reverse((value, _, _))) = heap.peek() {
                if value >= query {
                    break;
                }
                let Reverse((_, r, c)) = heap.pop().unwrap();
                count += 1;
                for (dr, dc) in Self::DIRS {
                    let (r, c) = (dr + r, dc + c);
                    if 0 <= r && r < m && 0 <= c && c < n {
                        let v = grid[r as usize][c as usize];
                        if v > 0 {
                            heap.push(Reverse((v, r, c)));
                            grid[r as usize][c as usize] = 0;
                        }
                    }
                }
            }
            output[index] = count;
        }

        output
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2,3],vec![2,5,7],vec![3,5,1]], vec![5,6,2], vec![5,8,1])]
    #[case(vec![vec![5,2,1],vec![1,1,2]], vec![3], vec![0])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] queries: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::max_points(grid, queries);
        assert_eq!(actual, expected);
    }
}
