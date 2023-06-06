//! Solution for https://leetcode.com/problems/count-and-say
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut say = std::collections::VecDeque::with_capacity(60);
        say.push_front(1);
        for _ in 2..=n {
            let (mut count, mut prev) = (0, 0);
            let len = say.len();
            for _ in 0..len {
                let cur = say.pop_front().unwrap();
                if cur == prev {
                    count += 1;
                } else {
                    if count != 0 {
                        say.push_back(count);
                        say.push_back(prev);
                    }
                    prev = cur;
                    count = 1;
                }
            }
            say.push_back(count);
            say.push_back(prev);
        }
        say.into_iter().map(|x| x.to_string()).collect()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(1, "1")]
    #[case::leet2(4, "1211")]
    fn test(#[case] n: i32, #[case] output: &str) {
        assert_eq!(Solution::count_and_say(n), output);
    }
}
