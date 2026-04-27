//! Solution for https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid
//! 1391. Check if There is a Valid Path in a Grid
//!
// 1 which means a street connecting the left cell and the right cell.
// 2 which means a street connecting the upper cell and the lower cell.
// 3 which means a street connecting the left cell and the lower cell.
// 4 which means a street connecting the right cell and the lower cell.
// 5 which means a street connecting the left cell and the upper cell.
// 6 which means a street connecting the right cell and the upper cell.

impl Solution {
    const DIR_CHANGE: [[Option<usize>; 4]; 6] = [
        // TOP , BOT , LEF , RIG
        [None, None, Some(2), Some(3)],
        [Some(0), Some(1), None, None],
        [Some(3), None, Some(0), None],
        [None, Some(2), None, Some(0)],
        [Some(3), None, Some(1), None],
        [Some(2), None, None, Some(1)],
    ];

    const DELTA_CHANGE: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn _has_valid_path(
        mut x: usize,
        mut y: usize,
        mut dir: usize,
        m: usize,
        n: usize,
        grid: &[Vec<i32>],
    ) -> bool {
        let mut start = false;
        loop {
            if let Some(r) = grid.get(y)
                && let Some(c) = r.get(x)
                && let Some(d) = Self::DIR_CHANGE[(c - 1) as usize][dir]
            {
                if start && x == 0 && y == 0 {
                    break false;
                }
                start = true;
                let (dx, dy) = Self::DELTA_CHANGE[d];
                if (x == 0 && dx == -1) || (y == 0 && dy == -1) {
                    break false;
                }
                if x == n && y == m {
                    break true;
                }
                x = (x as isize + dx) as _;
                y = (y as isize + dy) as _;
                dir = d;
            } else {
                break false;
            }
        }
    }

    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;
        Self::_has_valid_path(0, 0, 0, m, n, &grid)
            || Self::_has_valid_path(0, 0, 2, m, n, &grid)
            || Self::_has_valid_path(0, 0, 1, m, n, &grid)
            || Self::_has_valid_path(0, 0, 3, m, n, &grid)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1]],true)]
    #[case(vec![vec![2,4,3],vec![6,5,2]],true)]
    #[case(vec![vec![2,4,3],vec![6,5,2]],true)]
    #[case(vec![vec![2,1,4,3],vec![6,1,5,2]],true)]
    #[case(vec![vec![1,2,1],vec![1,2,1]],false)]
    #[case(vec![vec![1,1,2]], false)]
    #[case(vec![vec![4,1], vec![6,1]], true)]
    #[case(vec![vec![4,3, 3], vec![6,5, 2]], false)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::has_valid_path(grid);
        assert_eq!(actual, expected);
    }
}
