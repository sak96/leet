impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|i| i as _).unwrap_or(-1)
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1("sadbutsad", "sad", 0)]
    #[case::leet1("leetcode", "leeto", -1)]
    fn test(#[case] haystack: &str, #[case] needle: &str, #[case] expected: i32) {
        assert_eq!(Solution::str_str(haystack.into(), needle.into()), expected);
    }
}
