//! Solution for https://leetcode.com/problems/product-of-array-except-self
//! 238. Product of Array Except Self

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = Vec::with_capacity(nums.len());
        let mut temp = 1;
        for n in &nums {
            output.push(temp);
            temp *= n;
        }
        temp = 1;
        for (i, n) in nums.iter().enumerate().rev() {
            output[i] *= temp;
            temp *= n;
        }
        output
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4], vec![24,12,8,6])]
    #[case(vec![-1,1,0,-3,3], vec![0,0,9,0,0])]
    fn case(#[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::product_except_self(nums);
        assert_eq!(actual, expected);
    }
}
