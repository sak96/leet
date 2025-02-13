//! Solution for https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits
//! 2342. Max Sum of a Pair With Equal Sum of Digits

impl Solution {
    pub fn sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut map = vec![-1; 82];
        let mut max_sum = -1;
        for num in nums {
            let sum = Self::sum(num);
            let val = map[sum as usize];
            if val > 0 {
                max_sum = max_sum.max(num + val);
            }
            map[sum as usize] = val.max(num);
        }
        max_sum
    }
}
// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![18,43,36,13,7], 54)]
    #[case(vec![10,12,19,14], -1)]
    #[case(vec![229,398,269,317,420,464,491,218,439,153,482,169,411,93,147,50,347,210,251,366,401], 973)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::maximum_sum(nums);
        assert_eq!(actual, expected);
    }
}
