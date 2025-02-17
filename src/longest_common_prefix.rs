//! Solution for https://leetcode.com/problems/longest-common-prefix
//! 14. Longest Common Prefix

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let bytes: Vec<_> = strs.into_iter().map(|b| b.as_bytes().to_vec()).collect();
        let len = bytes.iter().map(|a| a.len()).min().unwrap();
        let (first, others) = bytes.split_first().unwrap();
        for i in 0..len {
            for b in others {
                if b[i] != first[i] {
                    return std::str::from_utf8(&b[0..i]).unwrap().to_string();
                }
            }
        }
        std::str::from_utf8(&first[0..len]).unwrap().to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec!["flower".into(),"flow".into(),"flight".into()], "fl".to_string())]
    #[case(vec!["dog".into(),"racecar".into(),"car".into()], "".to_string())]
    fn case(#[case] strs: Vec<String>, #[case] expected: String) {
        let actual = Solution::longest_common_prefix(strs);
        assert_eq!(actual, expected);
    }
}
