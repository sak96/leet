#![allow(dead_code)]
use std::collections::VecDeque;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix: VecDeque<VecDeque<_>> = matrix.into_iter().map(|v| v.into()).collect();
        let mut output = Vec::with_capacity(matrix.len() * matrix[0].len());
        while !matrix.is_empty() {
            // right
            if let Some(v) = matrix.pop_front() {
                output.extend(v);
            }
            // down
            for row in matrix.iter_mut() {
                if let Some(x) = row.pop_back() {
                    output.push(x);
                }
            }
            // left
            if let Some(v) = matrix.pop_back() {
                output.extend(v.iter().rev());
            }
            // up
            for row in matrix.iter_mut().rev() {
                if let Some(x) = row.pop_front() {
                    output.push(x);
                }
            }
        }
        output
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let output = [1, 2, 3, 6, 9, 8, 7, 4, 5];
        let matrix: Vec<Vec<_>> = matrix.iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::spiral_order(matrix), output);
    }

    #[test]
    fn case2() {
        let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let output = [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        let matrix: Vec<Vec<_>> = matrix.iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::spiral_order(matrix), output);
    }

    #[test]
    fn case3() {
        let matrix = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        let output = [1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10];
        let matrix: Vec<Vec<_>> = matrix.iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::spiral_order(matrix), output);
    }
}
