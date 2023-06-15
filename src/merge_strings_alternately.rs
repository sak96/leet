impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut output = String::with_capacity(word1.len() + word2.len());
        let mut word1 = word1.chars().peekable();
        let mut word2 = word2.chars().peekable();
        while word1.peek().is_some() && word2.peek().is_some() {
            output.push(word1.next().unwrap());
            output.push(word2.next().unwrap());
        }
        output.extend(word2);
        output.extend(word1);
        output
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    #[rstest]
    #[case::leet1("abc", "pqr", "apbqcr")]
    #[case::leet2("ab", "pqrs", "apbqrs")]
    #[case::leet3("abcd", "pq", "apbqcd")]
    fn test(#[case] word1: &str, #[case] word2: &str, #[case] expected: &str) {
        assert_eq!(
            Solution::merge_alternately(word1.into(), word2.into()),
            expected
        )
    }
}
