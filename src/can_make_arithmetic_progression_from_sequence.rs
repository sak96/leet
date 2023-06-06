//! Solution for https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let mut first = arr.pop().expect("2 <= arr.length");
        let second = arr.pop().expect("2 <= arr.length");
        let diff = second - first;
        first = second;
        while let Some(second) = arr.pop() {
            if second - first != diff {
                return false;
            }
            first = second;
        }
        true
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![3,5,1],true)]
    #[case::leet2(vec![1,2,4],false)]
    fn test(#[case] arr: Vec<i32>, #[case] expected: bool) {
        assert_eq!(Solution::can_make_arithmetic_progression(arr), expected);
    }
}
