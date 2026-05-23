//! Solution for https://leetcode.com/problems/check-if-array-is-sorted-and-rotated
//! 1752. Check if Array Is Sorted and Rotated

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        nums.iter()
            .zip(nums.iter().cycle().skip(1))
            .filter(|(a, b)| a > b)
            .count()
            <= 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,4,5,1,2], true)]
    #[case(vec![3,3,4,5,1,1,2,2], true)]
    #[case(vec![3,3,4,5,4,4], false)]
    #[case(vec![3,3,4,5,4,4], false)]
    #[case(vec![2,1,3,4], false)]
    #[case(vec![1,2,3], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::check(nums);
        assert_eq!(actual, expected);
    }
}
