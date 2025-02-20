//! Solution for https://leetcode.com/problems/find-unique-binary-string
//! 1980. Find Unique Binary String

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut output = String::with_capacity(nums.len());
        for (i, n) in nums.iter().enumerate() {
            output.push(if n.as_bytes()[i] == b'0' { '1' } else { '0' })
        }
        output
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["01".into(),"10".into()])]
    #[case(vec!["00".into(),"01".into()])]
    #[case(vec!["111".into(),"011".into(),"001".into()])]
    fn case(#[case] nums: Vec<String>) {
        let actual = Solution::find_different_binary_string(nums.clone());
        assert!(!nums.contains(&actual));
    }
}
