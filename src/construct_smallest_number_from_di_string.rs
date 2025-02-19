//! Solution for https://leetcode.com/problems/construct-smallest-number-from-di-string
//! 2375. Construct Smallest Number From DI String

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut output = vec![];
        let mut reverse = vec![];
        for (i, ch) in pattern.chars().enumerate() {
            let i = i as u8;
            match ch {
                'I' => {
                    output.push(i + b'1');
                    if !reverse.is_empty() {
                        output.extend(reverse.drain(..).rev());
                    }
                }
                _ => {
                    reverse.push(i + b'1');
                }
            }
        }
        output.push(pattern.len() as u8 + b'1');
        if !reverse.is_empty() {
            output.extend(reverse.drain(..).rev());
        }
        std::str::from_utf8(&output).unwrap().to_string()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("IIIDIDDD", "123549876")]
    #[case("DDD", "4321")]
    fn case(#[case] pattern: String, #[case] expected: String) {
        let actual = Solution::smallest_number(pattern);
        assert_eq!(actual, expected);
    }
}
