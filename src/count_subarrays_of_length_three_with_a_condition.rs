//! Solution for https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition
//! 3392. Count Subarrays of Length Three With a Condition

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (nums[0], nums[1]);
        nums.into_iter()
            .skip(2)
            .filter(|c| {
                let condition = 2 * (a + c) == b;
                a = b;
                b = *c;
                condition
            })
            .count() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1,4,1], 1)]
    #[case(vec![1,1,1], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::count_subarrays(nums);
        assert_eq!(actual, expected);
    }
}
