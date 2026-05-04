//! Solution for https://leetcode.com/problems/rotate-image
//! 48. Rotate Image

impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        let mut xmax = matrix.len() - 1;
        let mut xmin = 0;
        while xmin < xmax {
            for i in 0..(xmax - xmin) {
                let mut temp = matrix[xmin][xmin + i];
                std::mem::swap(&mut temp, &mut matrix[xmin + i][xmax]);
                std::mem::swap(&mut temp, &mut matrix[xmax][xmax - i]);
                std::mem::swap(&mut temp, &mut matrix[xmax - i][xmin]);
                std::mem::swap(&mut temp, &mut matrix[xmin][xmin + i]);
            }
            xmin += 1;
            xmax -= 1;
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
    #[case(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]], vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]])]
    #[case(vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]],  vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]])]
    fn case(#[case] mut matrix: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        dbg!(&matrix);
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
