//! Solution for https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer
//! 2529. Maximum Count of Positive Integer and Negative Integer

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let (mut pos, mut neg) = (0, 0);
        for i in nums {
            match i.cmp(&0) {
                std::cmp::Ordering::Less => neg += 1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => pos += 1,
            }
        }
        pos.max(neg)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![-2,-1,-1,1,2,3], 3)]
    #[case(vec![-3,-2,-1,0,0,1,2], 3)]
    #[case(vec![5,20,66,1314], 4)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::maximum_count(nums);
        assert_eq!(actual, expected);
    }
}
