//! Solution for https://leetcode.com/problems/single-number-ii
//! 137. Single Number II

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut ones, mut twos) = (0, 0);
        for num in nums {
            ones = (ones ^ num) & !twos; // xor of all items which have only appeared once
            twos = (twos ^ num) & !ones; // xor of all items which have only appeared twice
        }
        ones
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,2,3,2], 3)]
    #[case(vec![0,1,0,1,0,1,99], 99)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::single_number(nums);
        assert_eq!(actual, expected);
    }
}
