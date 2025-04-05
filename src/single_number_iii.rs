//! Solution for https://leetcode.com/problems/single-number-iii
//! 260. Single Number III

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let a_xor_b = nums.iter().fold(0, |acc, i| acc ^ i);
        let a_bit = if a_xor_b == i32::MIN {
            -1
        } else {
            (a_xor_b & (a_xor_b - 1)) ^ a_xor_b
        };
        let a = nums
            .iter()
            .filter(|z| (**z & a_bit) > 0)
            .fold(0, |acc, i| acc ^ i);
        let b = a_xor_b ^ a;
        vec![a, b]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1,3,2,5], vec![3,5])]
    #[case(vec![-1,0], vec![-1,0])]
    #[case(vec![0,1], vec![0, 1])]
    #[case(vec![-1, 2147483647], vec![-1, 2147483647])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let mut actual = Solution::single_number(nums);
        actual.sort_unstable();
        assert_eq!(actual, expected);
    }
}
