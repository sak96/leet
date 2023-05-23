impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max = 0;
        let mut count = 0;
        let mut stack = vec![];
        for c in s.chars() {
            if c == '(' {
                max = max.max(count);
                stack.push(count);
                count = 0;
            }
            if c == ')' {
                if let Some(x) = stack.pop() {
                    count += 2 + x;
                } else {
                    max = max.max(count);
                    count = 0;
                    stack.truncate(0);
                }
            }
        }
        max.max(count)
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(()", 2)]
    #[case(")()())", 4)]
    #[case("", 0)]
    #[case("(())", 4)]
    #[case("()(()", 2)]
    #[case("(()()", 4)]
    #[case("(()()(())", 8)]
    #[case("(()()(())(", 8)]
    #[case("(()(((()", 2)]
    fn case(#[case] s: &str, #[case] expected: i32) {
        dbg!(s);
        assert_eq!(Solution::longest_valid_parentheses(s.to_string()), expected);
    }
}
