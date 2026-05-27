//! Solution for https://leetcode.com/problems/count-the-number-of-special-characters-ii
//! 3121. Count the Number of Special Characters II

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let word = word.into_bytes();
        let mut seen = [0u8; (b'z' + 1 - b'a') as usize];
        for i in word {
            if i < b'a' {
                let i = (i - b'A') as usize;
                if seen[i] == 1 {
                    seen[i] = 3
                } else if seen[i] != 3 {
                    seen[i] = 7
                }
            } else {
                let i = (i - b'a') as usize;
                if seen[i] == 0 {
                    seen[i] = 1
                } else if seen[i] != 1 {
                    seen[i] = 7
                }
            }
        }
        seen.into_iter().filter(|&x| x == 3).count() as i32
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
    #[case("AbBCab", 0)]
    #[case("ABC", 0)]
    fn case(#[case] word: String, #[case] expected: i32) {
        let actual = Solution::number_of_special_chars(word);
        assert_eq!(actual, expected);
    }
}
