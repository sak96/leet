//! Solution for https://leetcode.com/problems/maximum-subarray
//! 53. Maximum Subarray

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((i32::MIN, 0), |(maxsum, cursum), &num| {
                let cursum = num.max(cursum + num);
                (maxsum.max(cursum), cursum)
            })
            .0
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-2,1,-3,4,-1,2,1,-5,4], 6)]
    #[case(vec![1], 1)]
    #[case(vec![5,4,-1,7,8], 23)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_sub_array(nums);
        assert_eq!(actual, expected);
    }
}
