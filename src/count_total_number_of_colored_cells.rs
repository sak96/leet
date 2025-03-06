//! Solution for https://leetcode.com/problems/count-total-number-of-colored-cells
//! 2579. Count Total Number of Colored Cells

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        2 * n * n - 2 * n + 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(2, 5)]
    fn case(#[case] n: i32, #[case] expected: i64) {
        let actual = Solution::colored_cells(n);
        assert_eq!(actual, expected);
    }
}
