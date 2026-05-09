//! Solution for https://leetcode.com/problems/cyclically-rotating-a-grid
//! 1914. Cyclically Rotating a Grid

impl Solution {
    #[inline]
    pub fn get_index(i: usize, x_len: usize, y_len: usize, len: usize) -> (usize, usize) {
        let i = i % len;
        if i < y_len {
            (0, i)
        } else if i - y_len + 1 < x_len {
            (i - y_len + 1, y_len - 1)
        } else if len - i < x_len {
            (len - i, 0)
        } else {
            (x_len - 1, len - i - x_len + 1)
        }
    }

    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut xmin = 0;
        let mut x_len = grid[0].len();
        let mut ymin = 0;
        let mut y_len = grid.len();
        while x_len > 0 && y_len > 0 {
            let len = 2 * (x_len + y_len) - 4;
            let k = k % len;
            if k != 0 {
                let mut queue = std::collections::VecDeque::with_capacity(k);
                for i in (len - k)..len {
                    let (x1, y1) = Self::get_index(i, x_len, y_len, len);
                    queue.push_back(grid[y1 + ymin][x1 + xmin]);
                }
                for i in 0..len {
                    let val = queue.pop_front().unwrap();
                    let (x1, y1) = Self::get_index(i, x_len, y_len, len);
                    queue.push_back(grid[y1 + ymin][x1 + xmin]);
                    grid[y1 + ymin][x1 + xmin] = val;
                }
            }

            xmin += 1;
            x_len -= 2;
            ymin += 1;
            y_len -= 2;
        }
        grid
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![40,10],vec![30,20]], 1, vec![vec![10,20],vec![40,30]])]
    #[case(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16]], 2, vec![vec![3,4,8,12],vec![2,11,10,16],vec![1,7,6,15],vec![5,9,13,14]])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::rotate_grid(grid, k);
        assert_eq!(actual, expected);
    }
}
