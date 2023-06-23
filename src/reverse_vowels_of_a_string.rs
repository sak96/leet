//! Solution for https://leetcode.com/problems/reverse-vowels-of-a-string
impl Solution {
    const VOWELS: &'static [u8] = &[b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
    pub fn reverse_vowels(mut s: String) -> String {
        let vec = unsafe { s.as_mut_vec() };
        let mut end = vec.len() - 1;
        let mut start = 0;
        while end > start {
            while end > start {
                if Self::VOWELS.contains(&vec[start]) {
                    break;
                }
                start += 1;
            }
            while end > start {
                if Self::VOWELS.contains(&vec[end]) {
                    break;
                }
                end -= 1;
            }
            if end > start {
                let (a, b) = vec.split_at_mut(end);
                std::mem::swap(&mut a[start], &mut b[0]);
                start += 1;
                end -= 1;
            }
        }
        s
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::leet1("hello", "holle")]
    #[case::leet2("leetcode", "leotcede")]
    fn test(#[case] s: &str, #[case] out: &str) {
        assert_eq!(Solution::reverse_vowels(s.into()), out);
    }
}
