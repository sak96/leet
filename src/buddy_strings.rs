impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() == goal.len() {
            let (mut i1, mut i2) = (None, None);
            let mut seen = [0; 26];
            for (a, b) in s.chars().zip(goal.chars()) {
                if a != b {
                    match (i1, i2) {
                        (None, None) => i1 = Some((a, b)),
                        (Some((a1, b1)), None) if a1 == b && b1 == a => i2 = Some((a, b)),
                        _ => return false,
                    }
                } else {
                    seen[(a as u8 - b'a') as usize] += 1;
                }
            }
            return match (i1, i2) {
                (Some(_), Some(_)) => true,
                (None, None) => seen.iter().any(|x| x >= &2),
                _ => false,
            };
        }
        false
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ab", "ba", true)]
    #[case("ab", "ab", false)]
    #[case("aa", "aa", true)]
    fn test(#[case] s: &str, #[case] goal: &str, #[case] solution: bool) {
        assert_eq!(Solution::buddy_strings(s.into(), goal.into()), solution);
    }
}
