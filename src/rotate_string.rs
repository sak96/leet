//! Solution for https://leetcode.com/problems/rotate-string
//! 796. Rotate String

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            false
        } else {
            let s = s.as_bytes();
            let goal = goal.as_bytes();
            let len = s.len();
            'outer: for i in 0..len {
                for j in 0..len {
                    if s[j] != goal[(j + i) % len] {
                        continue 'outer;
                    }
                }
                return true;
            }
            false
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
    #[case("abcde", "cdeab", true)]
    #[case("abcde", "abced", false)]
    fn case(#[case] s: String, #[case] goal: String, #[case] expected: bool) {
        let actual = Solution::rotate_string(s, goal);
        assert_eq!(actual, expected);
    }
}
