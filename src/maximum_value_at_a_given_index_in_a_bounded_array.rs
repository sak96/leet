//! Solution for https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array
impl Solution {
    pub fn max_value(n: i32, index: i32, mut max_sum: i32) -> i32 {
        // remove the diff part of 1
        max_sum -= n;
        let mut left = 1;
        let mut right = max_sum + 1;
        while left < right {
            let mid = (left + right) / 2;
            let left_count = index.min(mid - 1);
            let right_count = (n - 1 - index).min(mid - 1);
            let left_sum = (2 * mid - left_count - 1) * left_count / 2;
            let right_sum = (2 * mid - right_count - 1) * right_count / 2;
            if left_sum + mid + right_sum > max_sum {
                right = mid
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(4, 2, 6, 2)]
    #[case::leet1(6, 1, 10, 3)]
    fn test(#[case] n: i32, #[case] index: i32, #[case] max_sum: i32, #[case] output: i32) {
        assert_eq!(Solution::max_value(n, index, max_sum), output);
    }
}
