//! Solution for https://leetcode.com/problems/maximum-number-of-jumps-to-reach-the-last-index
//! 2770. Maximum Number of Jumps to Reach the Last Index

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![-1; n];
        dp[0] = 0;
        for (i, n1) in nums.iter().enumerate().skip(1) {
            for (j, n2) in nums.iter().enumerate().take(i) {
                if dp[j] < 1 {
                    continue;
                }
                if (n1 - n2).abs() <= target {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp[n - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    // #[case(vec![1,3,6,4,1,2], 2, 3)]
    // #[case(vec![1,3,6,4,1,2], 3, 5)]
    // #[case(vec![1,3,6,4,1,2], 0, -1)]
    #[case(vec![0,2,1,3], 1, -1)]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        let actual = Solution::maximum_jumps(nums, target);
        assert_eq!(actual, expected);
    }
}
