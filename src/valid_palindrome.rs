//! Solution for https://leetcode.com/problems/valid-palindrome
//! 125. Valid Palindrome

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut bytes = std::collections::VecDeque::from(s.as_bytes().to_vec());
        while !bytes.is_empty() {
            let ch1 = loop {
                match bytes.pop_front() {
                    Some(ch) if ch.is_ascii_alphanumeric() => {
                        break ch.to_ascii_lowercase();
                    }
                    None => return true,
                    _ => {}
                }
            };
            let ch2 = loop {
                match bytes.pop_back() {
                    Some(ch) if ch.is_ascii_alphanumeric() => {
                        break ch.to_ascii_lowercase();
                    }
                    None => return true,
                    _ => {}
                }
            };
            if ch1 != ch2 {
                return false;
            }
        }
        true
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
