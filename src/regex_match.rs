#[derive(Debug, PartialEq, Eq)]
pub struct RegexNode {
    pub letter: Option<u8>,
    pub star: bool,
}

impl Solution {
    pub fn matches(mut string: &[u8], mut pattern: &[RegexNode]) -> bool {
        let mut state = if let Some((first, pat)) = pattern.split_first() {
            pattern = pat;
            first
        } else {
            // state empty  => string should be empty
            return string.is_empty();
        };
        while let Some((letter, rest)) = string.split_first() {
            // if star check if the pattern matches skipping the state
            if state.star && Self::matches(string, pattern) {
                return true;
            }

            let letter_match =
                state.letter.is_none() || matches!(state.letter, Some(c)  if *letter == c);
            // if letter does not match it would not match even for star as we checked above
            if !letter_match {
                return false;
            }
            // consume state if not star
            if !state.star {
                state = if let Some((first, pat)) = pattern.split_first() {
                    pattern = pat;
                    first
                } else {
                    // state empty  => string should be empty
                    return rest.is_empty();
                };
            }

            // consume the letter
            string = rest;
        }
        state.star && pattern.iter().all(|x| x.star)
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut pattern = Vec::with_capacity(p.len());
        for i in p.chars() {
            match i {
                '.' => {
                    pattern.push(RegexNode {
                        letter: None,
                        star: false,
                    });
                }
                '*' => {
                    let mut last_char = pattern.pop().expect("* comes only after char");
                    last_char.star = true;
                    if !matches!(pattern.last_mut(), Some(prev) if last_char.eq(prev)) {
                        pattern.push(last_char);
                    }
                }
                x => {
                    pattern.push(RegexNode {
                        letter: Some(x as u8),
                        star: false,
                    });
                }
            }
        }
        Self::matches(s.as_ref(), &pattern)
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "aa";
        let p = "a";
        let output = false;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case2() {
        let s = "aa";
        let p = "a*";
        let output = true;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case3() {
        let s = "ab";
        let p = ".*";
        let output = true;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case4() {
        let s = "aab";
        let p = "c*a*b";
        let output = true;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case5() {
        let s = "";
        let p = ".*";
        let output = true;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case6() {
        let s = "aa";
        let p = "aaa";
        let output = false;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case7() {
        let s = "aaaa";
        let p = "aaa";
        let output = false;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case8() {
        let s = "aaaaaaaaaaaaab";
        let p = "a*a*a*a*a*a*a*a*a*c";
        let output = false;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case9() {
        let s = "aabcbcbcaccbcaabc";
        let p = ".*a*aa*.*b*.c*.*a*";
        let output = true;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case10() {
        let s = "bab";
        let p = "..*";
        let output = true;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }

    #[test]
    fn case11() {
        let s = "";
        let p = "";
        let output = true;
        assert_eq!(Solution::is_match(s.into(), p.into()), output)
    }
}
