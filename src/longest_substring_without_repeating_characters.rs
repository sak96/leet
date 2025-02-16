//! Solution for https://leetcode.com/problems/longest-substring-without-repeating-characters
//! 3. Longest Substring Without Repeating Characters

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen = [0; 256];
        let mut longest = 0;
        let mut current = 0;
        let s = s.as_bytes();
        for (idx, ch) in s.iter().enumerate() {
            let ch = *ch as usize;
            let idx = idx + 1;
            if seen[ch] == 0 {
                current += 1;
            } else {
                let old_idx = seen[ch];
                longest = longest.max(current);
                for i in seen.iter_mut() {
                    if old_idx.gt(i) {
                        *i = 0;
                    }
                }
                current = idx - old_idx;
            }
            seen[ch] = idx;
        }
        longest.max(current) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcabcbb", 3)]
    #[case("bbbbb", 1)]
    #[case("pwwkew", 3)]
    #[case("abba", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::length_of_longest_substring(s);
        assert_eq!(actual, expected);
    }
}
