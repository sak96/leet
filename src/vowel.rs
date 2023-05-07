impl Solution {
    pub fn is_vowel(c: u8) -> bool {
        matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
    }

    #[allow(dead_code)]
    pub fn max_vowels(mut s: String, k: i32) -> i32 {
        // dbg!(&s, &k);
        let k = k as usize;
        let s = unsafe { s.as_mut_vec() };

        let mut cur_vowels = s[..k].iter().filter(|c| Solution::is_vowel(**c)).count() as i32;
        let mut max_vowels = cur_vowels;

        for (start, end) in (k..(s.len())).map(|i| (i - k, i)) {
            if Solution::is_vowel(s[end]) {
                cur_vowels += 1;
            }
            if Solution::is_vowel(s[start]) {
                cur_vowels -= 1;
            }
            if cur_vowels == k as i32 {
                break;
            }
            max_vowels = std::cmp::max(max_vowels, cur_vowels);
        }
        std::cmp::max(max_vowels, cur_vowels)
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_vowels("abciiidef".into(), 3), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_vowels("aeiou".into(), 2), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::max_vowels("leetcode".into(), 3), 2);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::max_vowels("weallloveyou".into(), 7), 4);
    }
}
