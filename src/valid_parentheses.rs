//! Solution for https://leetcode.com/problems/valid-parentheses
//! 20. Valid Parentheses

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut memory = Vec::with_capacity((s.len() + 1) / 2);
        for ch in s.chars() {
            match ch {
                '{' | '(' | '[' => {
                    memory.push(ch);
                }
                '}' => {
                    if !matches!(memory.pop(), Some('{')) {
                        return false;
                    }
                }
                ')' => {
                    if !matches!(memory.pop(), Some('(')) {
                        return false;
                    }
                }
                ']' => {
                    if !matches!(memory.pop(), Some('[')) {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        memory.is_empty()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("()", true)]
    #[case("()[]{}", true)]
    #[case("(]", false)]
    #[case("([])", true)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::is_valid(s);
        assert_eq!(actual, expected);
    }
}
