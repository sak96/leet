//! Solution for https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii
//! 3306. Count of Substrings Containing Every Vowel and K Consonants II

impl Solution {
    const VOWEL: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
    pub fn _atleast_k(words: &[u8], k: i32) -> i64 {
        let mut count = 0;
        let mut start = 0;
        let mut vowel_count: [u8; 5] = [0; 5];
        let (mut constants, mut vowels) = (0, 0);
        for (end, c) in words.iter().enumerate() {
            if let Some(i) = Self::VOWEL.iter().position(|x| x.eq(c)) {
                if vowel_count[i] == 0 {
                    vowels += 1;
                }
                vowel_count[i] += 1;
            } else {
                constants += 1;
            }
            while constants >= k && vowels == 5 {
                count += words.len() - end;
                if let Some(i) = Self::VOWEL.iter().position(|x| x.eq(&words[start])) {
                    vowel_count[i] -= 1;
                    if vowel_count[i] == 0 {
                        vowels -= 1;
                    }
                } else {
                    constants -= 1;
                }
                start += 1;
            }
        }
        count as _
    }

    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let words = word.as_bytes();
        Self::_atleast_k(words, k) - Self::_atleast_k(words, k + 1)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aeioqq", 1, 0)]
    #[case("aeiou", 0, 1)]
    #[case("ieaouqqieaouqq", 1, 3)]
    fn case(#[case] word: String, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::count_of_substrings(word, k);
        assert_eq!(actual, expected);
    }
}
