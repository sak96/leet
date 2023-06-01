impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as _
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![1,3,5,6], 5, 2)]
    #[case::leet2(vec![1,3,5,6], 2, 1)]
    #[case::leet1(vec![1,3,5,6], 7, 4)]
    fn test(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: i32) {
        assert_eq!(Solution::search_insert(nums, target), expected);
    }
}
