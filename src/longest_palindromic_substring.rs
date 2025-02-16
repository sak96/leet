//! Solution for https://leetcode.com/problems/longest-palindromic-substring
//! 5. Longest Palindromic Substring

impl Solution {
    pub fn longest_palindrome(input: String) -> String {
        let s = input.as_bytes();
        let len = s.len();
        let mut palindrome = 0..0;
        for index in 0..len {
            let (mut i, mut j) = (index, index);
            while i > 0 && s[i] == s[i - 1] {
                i -= 1;
            }
            while j > len - 1 && s[j] == s[j + 1] {
                j += 1;
            }
            while i > 0 && j < len - 1 && s[i - 1] == s[j + 1] {
                i -= 1;
                j += 1;
            }
            let new_range = i..(j + 1);
            if palindrome.len() < new_range.len() {
                palindrome = new_range;
            }
        }
        input[palindrome].to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("babad", "bab")]
    #[case("cbbd", "bb")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::longest_palindrome(s);
        assert_eq!(actual, expected);
    }
}
