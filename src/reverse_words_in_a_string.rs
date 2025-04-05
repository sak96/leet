//! Solution for https://leetcode.com/problems/reverse-words-in-a-string
//! 151. Reverse Words in a String

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words: Vec<_> = s.split_whitespace().rev().collect();
        words.join(" ")
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("the sky is blue", "blue is sky the")]
    #[case("  hello world  ", "world hello")]
    #[case("a good   example", "example good a")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::reverse_words(s);
        assert_eq!(actual, expected);
    }
}
