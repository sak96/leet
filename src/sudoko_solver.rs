impl Solution {
    const NUMBER: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    const EMPTY: char = '.';

    #[inline]
    pub fn is_valid(board: &[Vec<char>], x: usize, y: usize) -> bool {
        let num = board[x][y];
        // check col + row
        for (r, row) in board.iter().enumerate() {
            if r == x {
                // check col if current row
                for (c, cell) in row.iter().enumerate() {
                    if cell == &num && c != y {
                        return false;
                    }
                }
            } else if row[y] == num {
                // if other row
                return false;
            }
        }
        // check square
        for (r, row) in board.iter().enumerate().skip((x / 3) * 3).take(3) {
            for (c, cell) in row.iter().enumerate().skip((y / 3) * 3).take(3) {
                if cell == &num && c != y && r != x {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve_cell(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
        let (next_row, next_col) = if col == 8 {
            (row + 1, 0)
        } else if row == 9 {
            // all rows filled
            return true;
        } else {
            (row, col + 1)
        };
        if board[row][col] == Self::EMPTY {
            for n in Self::NUMBER {
                board[row][col] = n;
                if Self::is_valid(board, row, col) && Self::solve_cell(board, next_row, next_col) {
                    return true;
                }
            }
            board[row][col] = Self::EMPTY;
            false
        } else {
            Self::solve_cell(board, next_row, next_col)
        }
    }
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve_cell(board, 0, 0);
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::graph_builder;

    use rstest::rstest;
    #[rstest]
    #[case::leetcode(
        graph_builder![
            ['5','3','.','.','7','.','.','.','.'],
            ['6','.','.','1','9','5','.','.','.'],
            ['.','9','8','.','.','.','.','6','.'],
            ['8','.','.','.','6','.','.','.','3'],
            ['4','.','.','8','.','3','.','.','1'],
            ['7','.','.','.','2','.','.','.','6'],
            ['.','6','.','.','.','.','2','8','.'],
            ['.','.','.','4','1','9','.','.','5'],
            ['.','.','.','.','8','.','.','7','9'],
        ],
        graph_builder![
            ['5','3','4','6','7','8','9','1','2'],
            ['6','7','2','1','9','5','3','4','8'],
            ['1','9','8','3','4','2','5','6','7'],
            ['8','5','9','7','6','1','4','2','3'],
            ['4','2','6','8','5','3','7','9','1'],
            ['7','1','3','9','2','4','8','5','6'],
            ['9','6','1','5','3','7','2','8','4'],
            ['2','8','7','4','1','9','6','3','5'],
            ['3','4','5','2','8','6','1','7','9'],
        ]
    )]
    #[case::last_cell(
        graph_builder![
            ['5','3','4','6','7','8','9','1','2'],
            ['6','7','2','1','9','5','3','4','8'],
            ['1','9','8','3','4','2','5','6','7'],
            ['8','5','9','7','6','1','4','2','3'],
            ['4','2','6','8','5','3','7','9','1'],
            ['7','1','3','9','2','4','8','5','6'],
            ['9','6','1','5','3','7','2','8','4'],
            ['2','8','7','4','1','9','6','3','5'],
            ['3','4','5','2','8','6','1','7','.'],
        ],
        graph_builder![
            ['5','3','4','6','7','8','9','1','2'],
            ['6','7','2','1','9','5','3','4','8'],
            ['1','9','8','3','4','2','5','6','7'],
            ['8','5','9','7','6','1','4','2','3'],
            ['4','2','6','8','5','3','7','9','1'],
            ['7','1','3','9','2','4','8','5','6'],
            ['9','6','1','5','3','7','2','8','4'],
            ['2','8','7','4','1','9','6','3','5'],
            ['3','4','5','2','8','6','1','7','9'],
        ]
    )]
    fn case(#[case] mut board: Vec<Vec<char>>, #[case] output: Vec<Vec<char>>) {
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, output);
    }
}
