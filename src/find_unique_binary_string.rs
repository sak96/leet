//! Solution for https://leetcode.com/problems/find-unique-binary-string
//! 1980. Find Unique Binary String

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut nums: Vec<_> = nums
            .into_iter()
            .map(|n| u16::from_str_radix(&n, 2).unwrap())
            .collect();
        nums.sort();
        let mut value = nums.len();
        for i in 0..nums.len() {
            if nums[i] != i as u16 {
                value = i;
                break;
            }
        }
        format!("{:0align$b}", value, align = nums.len())
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["01".into(),"10".into()], "00")]
    #[case(vec!["00".into(),"01".into()], "10")]
    #[case(vec!["111".into(),"011".into(),"001".into()], "000")]
    fn case(#[case] nums: Vec<String>, #[case] expected: String) {
        let actual = Solution::find_different_binary_string(nums);
        assert_eq!(actual, expected);
    }
}
