//! Solution for https://leetcode.com/problems/find-smallest-letter-greater-than-target
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let index = letters
            .binary_search(&((target as u8 + 1) as _))
            .unwrap_or_else(|x| x)
            % letters.len();
        letters[index]
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec!['c','f','j'], 'a', 'c')]
    #[case::leet1(vec!['c','f','j'], 'c', 'f')]
    #[case::leet1(vec!['x','x','y','y'], 'z', 'x')]
    fn test(#[case] letters: Vec<char>, #[case] target: char, #[case] expected: char) {
        assert_eq!(Solution::next_greatest_letter(letters, target), expected)
    }
}
