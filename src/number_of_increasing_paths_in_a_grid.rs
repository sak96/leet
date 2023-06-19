//! Solution for https://leetcode.com/problems/number-of-increasing-paths-in-a-grid
impl Solution {
    const MOD: i32 = 1e9 as i32 + 7;
    const NEIGHBOUR: &'static [[isize; 2]] = &[[0, 1], [0, -1], [1, 0], [-1, 0]];
    pub fn count_paths_for_cell(row: usize, col: usize, grid: &[Vec<i32>], dp: &mut Vec<Vec<i32>>) {
        if dp[row][col] > 0 {
            return;
        }
        let mut count = 1;
        for [r, c] in Self::NEIGHBOUR {
            if (row == 0 && r.is_negative()) || (col == 0 && c.is_negative()) {
                continue;
            }
            let r = (row as isize + r) as usize;
            let c = (col as isize + c) as usize;
            if let Some(value) = grid.get(r).map(|x| x.get(c)).flatten() {
                if value > &grid[row][col] {
                    Self::count_paths_for_cell(r, c, grid, dp);
                    count += dp[r][c];
                    count %= Self::MOD;
                }
            }
        }
        dp[row][col] = count;
    }

    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                Self::count_paths_for_cell(i, j, &grid, &mut dp);
                count += dp[i][j];
                count %= Self::MOD;
            }
        }
        count
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_builder;
    use rstest::rstest;

    #[rstest]
    #[case(graph_builder![[1,1],[3,4]],8)]
    #[case(graph_builder![[1],[2]],3)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] output: i32) {
        assert_eq!(Solution::count_paths(grid), output);
    }
}
