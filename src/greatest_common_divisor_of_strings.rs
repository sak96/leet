//! Solution for https://leetcode.com/problems/greatest-common-divisor-of-strings
impl Solution {
    pub fn gcd(min: usize, max: usize) -> usize {
        if min == 0 {
            max
        } else {
            Self::gcd(max % min, min)
        }
    }
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let product = format!("{str1}{str2}");
        let gcd = if product.eq(&format!("{str2}{str1}")) {
            Self::gcd(str1.len(), str2.len())
        } else {
            0
        };
        (&product[..gcd]).into()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1("ABCABC", "ABC", "ABC")]
    #[case::leet1("ABABAB", "ABAB", "AB")]
    #[case::leet1("LEET", "CODE", "")]
    fn test(#[case] str1: &str, #[case] str2: &str, #[case] gcd: &str) {
        assert_eq!(
            Solution::gcd_of_strings(str1.to_string(), str2.to_string()),
            gcd
        );
    }
}
