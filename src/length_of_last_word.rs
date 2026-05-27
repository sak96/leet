//! Solution for https://leetcode.com/problems/length-of-last-word
//! 58. Length of Last Word

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let p = s.into_bytes();
        let mut it = p.into_iter().rev().peekable();
        while let Some(b' ') = it.peek() {
            it.next();
        }
        let mut count = 0i32;
        for a in it {
            if a == b' ' {
                break;
            }
            count += 1
        }
        count
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("Hello World", 5)]
    #[case("   fly me   to   the moon  ", 4)]
    #[case("luffy is still joyboy", 6)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::length_of_last_word(s);
        assert_eq!(actual, expected);
    }
}
