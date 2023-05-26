impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut output = vec![];
        if n == 0 {
            output.push("".to_string());
        } else {
            for (l, r) in (0..n).zip((0..n).rev()) {
                for left in Self::generate_parenthesis(l) {
                    for right in Self::generate_parenthesis(r) {
                        output.push(format!("({left}){right}"));
                    }
                }
            }
        }
        output
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(3, vec!["((()))","(()())","(())()","()(())","()()()"])]
    #[case::leet2(1,  vec!["()"])]
    pub fn test(#[case] n: i32, #[case] mut expected: Vec<&str>) {
        expected.sort_unstable();
        let mut output = Solution::generate_parenthesis(n);
        output.sort_unstable();
        assert_eq!(output, expected);
    }
}
