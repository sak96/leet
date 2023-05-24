impl Solution {
    const NUMBER: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    const EMPTY: char = '.';

    #[inline]
    #[allow(clippy::needless_range_loop)]
    pub fn is_valid(board: &[Vec<char>], row: usize, col: usize) -> bool {
        let num = board[row][col];
        //check col
        for r in 0..9 {
            if board[r][col] == num && r != row {
                return false;
            }
        }
        // check row
        for c in 0..9 {
            if board[row][c] == num && c != col {
                return false;
            }
        }
        // check square
        let r = (row / 3) * 3;
        for r in r..(r + 3) {
            let c = (col / 3) * 3;
            for c in c..(c + 3) {
                if board[r][c] == num && c != col && r != row {
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
