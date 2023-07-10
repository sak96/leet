//! Solution for https://leetcode.com/problems/substring-with-largest-variance
//! 2272. Substring With Largest Variance

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut map = vec![0; 26];
        for i in s.bytes() {
            map[(i - b'a') as usize] += 1;
        }
        let mut ans = 0;
        for major in map
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (v > 0).then(|| (i as u8 + b'a')))
        {
            for (minor, mut remain_minor) in map
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| (v > 0).then(|| (i as u8 + b'a', v)))
            {
                if major == minor {
                    continue;
                }
                let (mut count_major, mut count_minor) = (0, 0);
                for ch in s.bytes() {
                    if ch == major {
                        count_major += 1;
                    } else if ch == minor {
                        count_minor += 1;
                        remain_minor -= 1;
                    }
                    if count_minor > 0 {
                        ans = ans.max(count_major - count_minor);
                    }
                    if count_major < count_minor && remain_minor > 0 {
                        count_major = 0;
                        count_minor = 0;
                    }
                }
            }
        }
        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aababbb", 3)]
    #[case("abcde", 0)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::largest_variance(s);
        assert_eq!(actual, expected);
    }
}
