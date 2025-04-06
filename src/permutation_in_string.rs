//! Solution for https://leetcode.com/problems/permutation-in-string
//! 567. Permutation in String

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let (mut s1_counter, mut s2_counter) = ([0; 26], [0; 26]);
        for i in 0..s1.len() {
            s1_counter[(s1[i] - b'a') as usize] += 1;
            s2_counter[(s2[i] - b'a') as usize] += 1;
        }
        let mut count = (0..26).filter(|&i| s1_counter[i] == s2_counter[i]).count();
        for (&c1, &c2) in s2.iter().zip(s2.iter().skip(s1.len())) {
            if count == 26 {
                return true;
            }
            let (l, h) = ((c1 - b'a') as usize, (c2 - b'a') as usize);
            s2_counter[l] -= 1;
            if s2_counter[l] == s1_counter[l] {
                count += 1;
            } else if s2_counter[l] == s1_counter[l] - 1 {
                count -= 1;
            }
            s2_counter[h] += 1;
            if s2_counter[h] == s1_counter[h] {
                count += 1;
            } else if s2_counter[h] == s1_counter[h] + 1 {
                count -= 1;
            }
        }
        count == 26
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ab", "eidbaooo", true)]
    #[case("ab", "eidboaoo", false)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        let actual = Solution::check_inclusion(s1, s2);
        assert_eq!(actual, expected);
    }
}
