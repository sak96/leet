//! Solution for https://leetcode.com/problems/find-missing-and-repeated-values
//! 2965. Find Missing and Repeated Values

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sq_sum = 0i64;
        let mut sum = 0i64;
        let n = (grid.len() * grid.len()) as i64;
        for r in grid {
            for c in r {
                sq_sum += (c * c) as i64;
                sum += (c) as i64;
            }
        }
        let expected_sq_sum = (2 * n + 1) * (n + 1) * n / 6;
        let expected_sum = (n) * (n + 1) / 2;
        let a_sq_minus_b_sq = sq_sum - expected_sq_sum;
        let a_minus_b = sum - expected_sum;

        let a_plus_b = a_sq_minus_b_sq / a_minus_b;
        let a = (a_plus_b + a_minus_b) / 2;
        let b = a_plus_b - a;
        vec![a as i32, b as i32]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,3],vec![2,2]], vec![2, 4])]
    #[case(vec![vec![9,1,7],vec![8,9,2],vec![3,4,6]], vec![9, 5])]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(actual, expected);
    }
}
