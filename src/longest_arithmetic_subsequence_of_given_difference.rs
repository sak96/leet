//! Solution for https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference
//! 1218. Longest Arithmetic Subsequence of Given Difference

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut previous_seq = std::collections::HashMap::new();
        arr.into_iter()
            .map(|i| {
                let new_seq_len = previous_seq.get(&(i - difference)).copied().unwrap_or(0) + 1;
                previous_seq.insert(i, new_seq_len);
                new_seq_len
            })
            .max()
            .unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4], 1, 4)]
    #[case(vec![1,3,5,7], 1, 1)]
    #[case(vec![1,5,7,8,5,3,4,2,1], -2, 4)]
    fn case(#[case] arr: Vec<i32>, #[case] difference: i32, #[case] expected: i32) {
        let actual = Solution::longest_subsequence(arr, difference);
        assert_eq!(actual, expected);
    }
}
