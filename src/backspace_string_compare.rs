//! Solution for https://leetcode.com/problems/backspace-string-compare
//! 844. Backspace String Compare

impl Solution {
    pub fn handle_backspace<'a, I>(mut s: I) -> Option<&'a u8>
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut back_space = 0;
        loop {
            match s.next() {
                Some(b'#') => back_space += 1,
                Some(_) if back_space > 0 => back_space -= 1,
                out => break out,
            }
        }
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s = s.as_bytes().iter().rev();
        let mut t = t.as_bytes().iter().rev();
        let (mut i, mut j) = (
            Self::handle_backspace(&mut s),
            Self::handle_backspace(&mut t),
        );
        loop {
            match (i, j) {
                (Some(a), Some(b)) if a == b => {
                    i = Self::handle_backspace(&mut s);
                    j = Self::handle_backspace(&mut t);
                }
                (None, None) => break true,
                _ => break false,
            }
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
    #[case("ab#c", "ad#c", true)]
    #[case("ab##", "c#d#", true)]
    #[case("a#c", "b", false)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: bool) {
        let actual = Solution::backspace_compare(s, t);
        assert_eq!(actual, expected);
    }
}
