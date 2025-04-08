//! Solution for https://leetcode.com/problems/partition-equal-subset-sum
//! 416. Partition Equal Subset Sum

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }
        let sum = (sum as usize) / 2;
        let mut dp = vec![false; sum + 1];
        dp[0] = true;
        for n in nums.iter().map(|&n| n as usize) {
            if n > sum {
                continue;
            }
            for i in (0..=(sum.saturating_sub(n))).rev() {
                if dp[i] {
                    dp[i + n] = true;
                }
            }
        }
        dp.pop().unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,5,11,5], true)]
    #[case(vec![1,2,5], false)]
    #[case(vec![1,2,3,5], false)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::can_partition(nums);
        assert_eq!(actual, expected);
    }
}
