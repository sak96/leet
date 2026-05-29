//! Solution for https://leetcode.com/problems/minimum-element-after-replacement-with-digit-sum
//! 3300. Minimum Element After Replacement With Digit Sum

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|mut x| {
                let mut sum = 0;
                while x > 0 {
                    sum += x % 10;
                    x /= 10;
                }
                sum
            })
            .min()
            .unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![10,12,13,14], 1)]
    #[case(vec![1,2,3,4], 1)]
    #[case(vec![999,19,199], 10)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_element(nums);
        assert_eq!(actual, expected);
    }
}
