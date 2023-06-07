//! Solution for https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let corrections = (a | b) ^ c;
        let flip_to_one = (corrections & c).count_ones();
        let flip_to_zero_in_a = (corrections & a).count_ones();
        let flip_to_zero_in_b = (corrections & b).count_ones();
        (flip_to_one + flip_to_zero_in_b + flip_to_zero_in_a) as i32
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(2, 6, 5, 3)]
    #[case::leet2(4, 2, 7, 1)]
    #[case::leet3(1, 2, 3, 0)]
    fn test(#[case] a: i32, #[case] b: i32, #[case] c: i32, #[case] flip: i32) {
        assert_eq!(Solution::min_flips(a, b, c), flip);
    }
}
