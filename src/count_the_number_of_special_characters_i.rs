//! Solution for https://leetcode.com/problems/count-the-number-of-special-characters-i
//! 3120. Count the Number of Special Characters I

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let bytes = word.as_bytes();
        let mut seen = [(false, false); (b'z' + 1 - b'a') as usize];
        for &b in bytes {
            if b >= b'a' {
                seen[(b - b'a') as usize].0 = true;
            } else {
                seen[(b - b'A') as usize].1 = true;
            }
        }
        seen.into_iter().filter(|f| f.0 && f.1).count() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aaAbcBC", 3)]
    #[case("abc", 0)]
    #[case("abBCab", 1)]
    fn case(#[case] word: String, #[case] expected: i32) {
        let actual = Solution::number_of_special_chars(word);
        assert_eq!(actual, expected);
    }
}
