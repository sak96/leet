//! Solution for https://leetcode.com/problems/maximize-the-confusion-of-an-exam
//! 2024. Maximize the Confusion of an Exam

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let answer_key = answer_key.as_bytes();
        let (mut max_size, mut t, mut f) = (0, 0, 0);
        for (i, &ch) in answer_key.iter().enumerate() {
            if ch == b'T' {
                t += 1;
            } else {
                f += 1;
            }
            if t.min(f) > k {
                if answer_key[i - (max_size as usize)] == b'T' {
                    t -= 1;
                } else {
                    f -= 1;
                }
            } else {
                max_size += 1;
            }
        }
        max_size
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("TTFF", 2, 4)]
    #[case("TFFT", 1, 3)]
    #[case("TTFTTFTT", 1, 5)]
    fn case(#[case] answer_key: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_consecutive_answers(answer_key, k);
        assert_eq!(actual, expected);
    }
}
