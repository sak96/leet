//! Solution for https://leetcode.com/problems/left-and-right-sum-differences
//! 2574. Left and Right Sum Differences

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = nums.iter().sum::<i32>();
        let mut answer = Vec::with_capacity(nums.len());
        let mut left_sum = 0;
        for i in nums {
            sum -= i;
            answer.push((sum - left_sum).abs());
            left_sum += i;
        }
        answer
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![10,4,8,3], vec![15,1,11,22])]
    #[case(vec![1], vec![0])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::left_right_difference(nums);
        assert_eq!(actual, expected);
    }
}
