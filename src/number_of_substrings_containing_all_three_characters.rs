//! Solution for https://leetcode.com/problems/number-of-substrings-containing-all-three-characters
//! 1358. Number of Substrings Containing All Three Characters

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut mem = [-1, -1, -1];
        let mut total = 0;

        for (i, &c) in s.as_bytes().iter().enumerate() {
            mem[(c - b'a') as usize] = i as i32;
            total += mem.iter().min().unwrap() + 1;
        }

        total
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcabc", 10)]
    #[case("aaacb", 3)]
    #[case("abc", 1)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::number_of_substrings(s);
        assert_eq!(actual, expected);
    }
}
