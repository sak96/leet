//! Solution for https://leetcode.com/problems/roman-to-integer
//! 13. Roman to Integer

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let b = s.as_bytes();
        let mut it = b.iter().peekable();
        let mut num = 0;
        loop {
            num += match it.next() {
                Some(&b'I') => {
                    if matches!(it.peek(), Some(&b'V') | Some(&b'X')) {
                        -1
                    } else {
                        1
                    }
                }
                Some(&b'V') => 5,
                Some(&b'X') => {
                    if matches!(it.peek(), Some(&b'L') | Some(&b'C')) {
                        -10
                    } else {
                        10
                    }
                }
                Some(&b'L') => 50,
                Some(&b'C') => {
                    if matches!(it.peek(), Some(&b'D') | Some(&b'M')) {
                        -100
                    } else {
                        100
                    }
                }
                Some(&b'D') => 500,
                Some(&b'M') => 1000,
                None => break num,
                _ => unreachable!(),
            };
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
    #[case("III", 3)]
    #[case("LVIII", 58)]
    #[case("MCMXCIV", 1994)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::roman_to_int(s);
        assert_eq!(actual, expected);
    }
}
