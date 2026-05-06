//! Solution for https://leetcode.com/problems/rotating-the-box
//! 1861. Rotating the Box

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();
        let mut rotated_box: Vec<Vec<char>> = vec![Vec::with_capacity(m); n];
        for row in box_grid.into_iter().rev() {
            let mut stones = 0;
            let mut fill_index = 0;
            for (y, v) in row.into_iter().enumerate() {
                match v {
                    '#' => stones += 1,
                    '*' => {
                        for i in fill_index..y - stones {
                            rotated_box[i].push('.');
                        }
                        for i in y - stones..y {
                            rotated_box[i].push('#');
                        }
                        rotated_box[y].push('*');
                        fill_index = y + 1;
                        stones = 0;
                    }
                    '.' => {}
                    _ => unreachable!(),
                };
            }
            for i in fill_index..n - stones {
                rotated_box[i].push('.');
            }
            for i in n - stones..n {
                rotated_box[i].push('#');
            }
        }
        rotated_box
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec!['#','.','#']], vec![vec!['.'], vec!['#'], vec!['#']])]
    #[case(vec![vec!['#','.','*']], vec![vec!['.'], vec!['#'], vec!['*']])]
    #[case(vec![vec!['#','.','*','.'],vec!['#','#','*','.']], vec![vec!['#','.'], vec!['#','#'], vec!['*','*'], vec!['.','.']])]
    #[case(vec![vec!['#','#','*','.','*','.'],vec!['#','#','#','*','.','.'],vec!['#','#','#','.','#','.']], vec![vec!['.','#','#'], vec!['.','#','#'], vec!['#','#','*'], vec!['#','*','.'], vec!['#','.','*'], vec!['#','.','.']])]
    fn case(#[case] box_grid: Vec<Vec<char>>, #[case] expected: Vec<Vec<char>>) {
        let actual = Solution::rotate_the_box(box_grid);
        assert_eq!(actual, expected);
    }
}
