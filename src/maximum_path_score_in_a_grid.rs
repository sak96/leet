//! Solution for https://leetcode.com/problems/maximum-path-score-in-a-grid
//! 3742. Maximum Path Score in a Grid

impl Solution {
    pub fn max_path_score_(
        grid: &[Vec<i32>],
        dp: &mut [Vec<Vec<i32>>],
        k: isize,
        x: usize,
        y: usize,
    ) -> i32 {
        if k < 0 {
            -1
        } else if x == 0 && y == 0 {
            // grid[0][0] == 0
            0
        } else if dp[y][x][k as usize] == i32::MIN {
            let mut score = 0;
            let mut cost = k;
            match grid[y][x] {
                0 => {}
                1 => {
                    cost -= 1;
                    score = 1;
                }
                2 => {
                    cost -= 1;
                    score = 2;
                }
                _ => unreachable!(),
            };
            let up = if y == 0 {
                i32::MIN
            } else {
                Self::max_path_score_(grid, dp, cost, x, y - 1)
            };
            let left = if x == 0 {
                i32::MIN
            } else {
                Self::max_path_score_(grid, dp, cost, x - 1, y)
            };
            let max = up.max(left);
            dp[y][x][k as usize] = if max >= 0 { max + score } else { -1 };
            dp[y][x][k as usize]
        } else {
            dp[y][x][k as usize]
        }
    }

    pub fn max_path_score(grid: Vec<Vec<i32>>, cost: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let cost = cost as usize;
        let mut dp = vec![vec![vec![i32::MIN; cost + 1]; n]; m];
        Self::max_path_score_(&grid, &mut dp, cost as isize, n - 1, m - 1).max(-1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0, 1],vec![2, 0]], 1, 2)]
    #[case(vec![vec![0, 1],vec![1, 2]], 1, -1)]
    #[case(vec![vec![0, 2, 1, 1, 0, 1, 0, 0, 2, 1, 1, 2, 1, 2, 0, 0, 2, 1, 1, 0, 1, 0, 2, 1, 0, 2, 2, 0, 2, 1, 1, 0, 1, 0, 2, 2, 0, 2, 1, 2, 1, 2, 1, 0, 0, 0, 1, 1, 2, 1, 1, 2, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 2, 2, 2, 0, 0, 1, 2, 0, 0, 1, 1, 2, 1, 2, 1, 1, 0, 2, 0, 0, 1],vec![0, 2, 0, 2, 1, 1, 0, 2, 1, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 0, 2, 2, 2, 0, 1, 1, 1, 2, 0, 2, 1, 0, 1, 1, 2, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 2, 1, 0, 0, 1, 2, 2, 0, 0, 0, 2, 1, 0, 2, 2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 0, 0, 1, 1, 2, 0, 0, 0, 1, 2, 1, 1, 1, 0, 1, 1, 2, 2, 0, 1],vec![0, 0, 2, 0, 0, 0, 1, 0, 2, 2, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 2, 0, 0, 2, 2, 0, 2, 0, 0, 1, 0, 1, 0, 0, 2, 2, 1, 0, 2, 0, 0, 2, 1, 0, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 2, 2, 0, 2, 2, 1, 2, 0, 2, 2, 0, 0, 2, 0, 2, 0, 2, 0, 1, 2, 2, 0, 1, 2, 2, 2, 0, 0, 1, 2, 0, 1, 2, 0, 0, 1, 0],vec![0, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 0, 0, 2, 1, 1, 1, 2, 2, 2, 0, 0, 2, 2, 2, 0, 1, 0, 0, 1, 0, 1, 1, 1, 2, 1, 0, 0, 1, 2, 0, 0, 2, 1, 1, 2, 0, 1, 1, 2, 0, 0, 0, 0, 1, 2, 2, 1, 2, 2, 0, 1, 0, 0, 2, 1, 2, 2, 2, 2, 2, 1, 0, 0, 1, 0, 0, 1, 1, 1, 2, 2, 0, 2, 2, 0, 0, 2, 1, 2, 2],vec![1, 0, 1, 1, 1, 1, 0, 2, 2, 2, 1, 2, 0, 1, 2, 0, 0, 1, 1, 1, 1, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 1, 2, 0, 2, 1, 0, 0, 1, 2, 1, 0, 2, 1, 0, 1, 0, 0, 1, 2, 2, 0, 0, 1, 0, 1, 0, 2, 2, 0, 1, 2, 1, 0, 2, 2, 0, 2, 1, 0, 2, 0, 2, 2, 2, 2, 2, 0, 2, 1, 0, 2, 0, 0, 2, 1, 1, 0],vec![1, 2, 0, 2, 1, 1, 1, 1, 0, 0, 2, 1, 0, 2, 2, 1, 0, 0, 1, 1, 0, 2, 2, 1, 0, 0, 2, 1, 1, 0, 0, 2, 2, 2, 2, 0, 0, 2, 1, 0, 1, 2, 2, 2, 2, 0, 0, 1, 2, 0, 2, 1, 2, 2, 1, 0, 1, 1, 1, 2, 1, 0, 1, 2, 0, 1, 0, 1, 0, 2, 2, 1, 1, 1, 1, 1, 0, 0, 1, 2, 0, 0, 2, 0, 1, 1, 1, 1, 2, 2, 0],vec![2, 1, 0, 1, 0, 0, 0, 0, 0, 0, 2, 0, 1, 2, 0, 2, 2, 0, 2, 0, 1, 0, 1, 0, 0, 2, 0, 0, 0, 0, 1, 0, 2, 1, 2, 1, 1, 0, 2, 0, 2, 0, 0, 2, 1, 0, 1, 2, 2, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 2, 1, 1, 1, 1, 2, 0, 1, 2, 0, 1, 1, 1, 2, 2, 1, 2, 2, 1, 2, 2, 0, 1, 2, 2, 1, 2, 0, 0, 2, 2]], 51, 0)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_path_score(grid, k);
        assert_eq!(actual, expected);
    }
}
