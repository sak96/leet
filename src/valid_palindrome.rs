//! Solution for https://leetcode.com/problems/valid-palindrome
//! 125. Valid Palindrome

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut bytes: std::collections::VecDeque<u8> = s
            .bytes()
            .filter_map(|c| c.is_ascii_alphanumeric().then_some(c.to_ascii_lowercase()))
            .collect();
        loop {
            let Some(ch1) = bytes.pop_front() else {
                break true;
            };
            let Some(ch2) = bytes.pop_back() else {
                break true;
            };
            dbg!(ch1 as char, ch2 as char);
            if ch1 != ch2 {
                break false;
            }
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("A man, a plan, a canal: Panama", true)]
    #[case("race a car", false)]
    #[case("0P", false)]
    // #[case(" ", true)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::is_palindrome(s.clone());
        assert_eq!(actual, expected, "{:?}", s);
    }
}
