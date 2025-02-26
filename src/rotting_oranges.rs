//! Solution for https://leetcode.com/problems/rotting-oranges
//! 994. Rotting Oranges

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut rotten = vec![];
        let mut fresh = 0;
        for (x, row) in grid.iter().enumerate() {
            for (y, &c) in row.iter().enumerate() {
                if c == 2 {
                    rotten.push((x, y));
                } else if c == 1 {
                    fresh += 1;
                }
            }
        }
        let mut next_rotten = vec![];
        let mut iteration = -1;
        while !rotten.is_empty() {
            for (x, y) in rotten.drain(..) {
                for (i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let Some(r) = x.checked_add_signed(i) else {
                        continue;
                    };
                    let Some(c) = y.checked_add_signed(j) else {
                        continue;
                    };
                    if r >= m || c >= n {
                        continue;
                    }
                    if grid[r][c] == 1 {
                        grid[r][c] = 2;
                        fresh -= 1;
                        next_rotten.push((r, c));
                    }
                }
            }
            iteration += 1;
            rotten = std::mem::take(&mut next_rotten);
        }
        if fresh > 0 {
            -1
        } else {
            iteration.max(0)
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,1,1],vec![1,1,0],vec![0,1,1]], 4)]
    #[case(vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]], -1)]
    #[case(vec![vec![0,2]], 0)]
    #[case(vec![vec![1,2]], 1)]
    #[case(vec![vec![1],vec![2],vec![1],vec![2]], 1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::oranges_rotting(grid);
        assert_eq!(actual, expected);
    }
}
