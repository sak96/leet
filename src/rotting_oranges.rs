//! Solution for https://leetcode.com/problems/rotting-oranges
//! 994. Rotting Oranges

impl Solution {
    pub fn check_rot(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> Option<(usize, usize)> {
        if (&grid.get(x).and_then(|row| row.get(y))) == &Some(&1) {
            grid[x][y] = 2;
            Some((x, y))
        } else {
            None
        }
    }

    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
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
                if let Some(p) = Self::check_rot(&mut grid, x + 1, y) {
                    next_rotten.push(p);
                    fresh -= 1;
                }
                if x != 0 {
                    if let Some(p) = Self::check_rot(&mut grid, x - 1, y) {
                        next_rotten.push(p);
                        fresh -= 1;
                    }
                }
                if y != 0 {
                    if let Some(p) = Self::check_rot(&mut grid, x, y - 1) {
                        next_rotten.push(p);
                        fresh -= 1;
                    }
                }
                if let Some(p) = Self::check_rot(&mut grid, x, y + 1) {
                    next_rotten.push(p);
                    fresh -= 1;
                }
            }
            iteration += 1;
            rotten = next_rotten.drain(..).collect();
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
