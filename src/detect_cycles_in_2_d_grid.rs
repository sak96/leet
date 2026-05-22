//! Solution for https://leetcode.com/problems/detect-cycles-in-2d-grid
//! 1559. Detect Cycles in 2D Grid

impl Solution {
    pub fn traverse(
        grid: &mut [Vec<char>],
        char: char,
        x: usize,
        y: usize,
        m: usize,
        n: usize,
        seen: &mut Vec<(usize, usize)>,
    ) -> bool {
        if x >= m || y >= n {
            false
        } else {
            match grid[x][y] {
                '1' => {
                    (seen
                        .iter()
                        .rev()
                        .position(|(i, j)| i == &x && j == &y)
                        .unwrap())
                        >= 3
                }
                c if c == char => {
                    grid[x][y] = '1';
                    seen.push((x, y));
                    let len = seen.len();
                    for (i, j) in [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)] {
                        if (x == 0 && i < 0) || (y == 0 && j < 0) {
                            continue;
                        }
                        seen.truncate(len);
                        if Self::traverse(
                            grid,
                            char,
                            (x as isize + i) as usize,
                            (y as isize + j) as usize,
                            m,
                            n,
                            seen,
                        ) {
                            return true;
                        }
                    }
                    grid[x][y] = '0';
                    false
                }
                _ => false,
            }
        }
    }

    pub fn contains_cycle(mut grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '0' {
                    continue;
                }
                let char = grid[i][j];
                if Self::traverse(&mut grid, char, i, j, m, n, &mut vec![]) {
                    return true;
                }
            }
        }
        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!['a','a','a','a'],vec!['a','b','b','a'],vec!['a','b','b','a'],vec!['a','a','a','a']], true)]
    #[case(vec![vec!['c','c','c','a'],vec!['c','d','c','c'],vec!['c','c','e','c'],vec!['f','c','c','c']], true)]
    #[case(vec![vec!['a','b','b'],vec!['b','z','b'],vec!['b','b','a']], false)]
    #[case(vec![vec!['a','a','b']], false)]
    #[case(vec![vec!['b','a','c'],vec!['c','a','c'],vec!['d','d','c'],vec!['b','c','c']], false)]
    #[case(vec![vec!['f','a','a','c','b'],vec!['e','a','a','e','c'],vec!['c','f','b','b','b'],vec!['c','e','a','b','e'],vec!['f','e','f','b','f']], true)]
    #[case(vec![vec!['f','a','a','c','b'],vec!['e','a','a','e','c']], true)]
    #[case(vec![vec!['f','a','a'],vec!['e','a','a']], true)]
    fn case(#[case] grid: Vec<Vec<char>>, #[case] expected: bool) {
        let actual = Solution::contains_cycle(grid);
        assert_eq!(actual, expected);
    }
}
