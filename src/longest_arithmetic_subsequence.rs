//! Solution for https://leetcode.com/problems/longest-arithmetic-subsequence
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in 1..nums.len() {
            for j in 0..i {
                let diff = nums[i] - nums[j];
                let value = *map.get(&(j, diff)).unwrap_or(&1);
                map.insert((i, diff), value + 1);
            }
        }
        map.into_values().max().unwrap()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1([3,6,9,12],4)]
    #[case::leet2([9,4,7,2,10],3)]
    #[case::leet3([20,1,15,3,10,5,8],4)]
    fn test(#[case] nums: impl AsRef<[i32]>, #[case] output: i32) {
        assert_eq!(
            Solution::longest_arith_seq_length(nums.as_ref().into()),
            output
        );
    }
}
