//! Solution for https://leetcode.com/problems/partition-labels
//! 763. Partition Labels

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut max_indices = [0; 26];
        for (i, c) in s.iter().enumerate() {
            max_indices[(c - b'a') as usize] = i;
        }
        let mut output = vec![];
        let mut end = 0;
        let mut start = 0;
        for (i, c) in s.iter().enumerate() {
            end = max_indices[(c - b'a') as usize].max(end);
            if end == i {
                output.push((end - start + 1) as i32);
                start = i + 1;
                end = i + 1;
            }
        }
        output
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ababcbacadefegdehijhklij", vec![9,7,8])]
    #[case("eccbbbbdec", vec![10])]
    fn case(#[case] s: String, #[case] expected: Vec<i32>) {
        let actual = Solution::partition_labels(s);
        assert_eq!(actual, expected);
    }
}
