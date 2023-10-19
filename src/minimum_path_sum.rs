//! Solution for https://leetcode.com/problems/minimum-path-sum
//! 64. Minimum Path Sum

impl Solution {
    const MAX: i32 = 8_00_00_01;
    pub fn min_path_sum_(grid: &[Vec<i32>], memory: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
        if memory[i][j] == Self::MAX {
            memory[i][j] = if i == 0 {
                Self::min_path_sum_(grid, memory, i, j - 1)
            } else if j == 0 {
                Self::min_path_sum_(grid, memory, i - 1, j)
            } else {
                let up = Self::min_path_sum_(grid, memory, i, j - 1);
                let left = Self::min_path_sum_(grid, memory, i - 1, j);
                up.min(left)
            } + grid[i][j]
        }
        memory[i][j]
    }
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut memory = vec![vec![Self::MAX; n]; m];
        memory[0][0] = grid[0][0];
        Self::min_path_sum_(&grid, &mut memory, m - 1, n - 1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]], 7)]
    #[case(vec![vec![1,2,3],vec![4,5,6]], 12)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_path_sum(grid);
        assert_eq!(actual, expected);
    }
}
