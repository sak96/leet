//! Solution for https://leetcode.com/problems/separate-the-digits-in-an-array
//! 2553. Separate the Digits in an Array

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::with_capacity(nums.len());
        let mut vec = Vec::with_capacity(6);
        for mut i in nums.into_iter() {
            if i == 0 {
                answer.push(0);
                continue;
            }
            while i > 0 {
                vec.push(i % 10);
                i /= 10;
            }
            answer.extend(vec.iter().rev());
            vec.clear();
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
    #[case(vec![13,25,83,77], vec![1,3,2,5,8,3,7,7])]
    #[case(vec![7,1,3,9], vec![7,1,3,9])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::separate_digits(nums);
        assert_eq!(actual, expected);
    }
}
