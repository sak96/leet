//! Solution for https://leetcode.com/problems/shortest-common-supersequence
//! 1092. Shortest Common Supersequence

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let (bytes1, bytes2) = (str1.as_bytes(), str2.as_bytes());
        let (mut len1, mut len2) = (str1.len(), str2.len());
        let mut mem = vec![vec![0; bytes2.len() + 1]; bytes1.len() + 1];

        for i in 1..=len1 {
            mem[i][0] = i;
        }
        for i in 0..=len2 {
            mem[0][i] = i;
        }
        for i in 1..=len1 {
            for j in 1..=len2 {
                mem[i][j] = if bytes1[i - 1] == bytes2[j - 1] {
                    mem[i - 1][j - 1]
                } else {
                    mem[i - 1][j].min(mem[i][j - 1])
                } + 1;
            }
        }
        let mut seq = vec![];
        while len1 > 0 && len2 > 0 {
            if bytes1[len1 - 1] == bytes2[len2 - 1] {
                len1 -= 1;
                len2 -= 1;
                seq.push(bytes1[len1]);
            } else if mem[len1 - 1][len2] < mem[len1][len2 - 1] {
                len1 -= 1;
                seq.push(bytes1[len1]);
            } else {
                len2 -= 1;
                seq.push(bytes2[len2]);
            }
        }
        while len1 > 0 {
            len1 -= 1;
            seq.push(bytes1[len1]);
        }
        while len2 > 0 {
            len2 -= 1;
            seq.push(bytes2[len2]);
        }
        seq.reverse();
        String::from_utf8(seq).unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abac", "cab", "cabac")]
    #[case("bbbaaaba", "bbababbb", "bbbaaababbb")]
    #[case("cab", "abac", "cabac")]
    #[case("aaaaaaaa", "aaaaaaaa", "aaaaaaaa")]
    fn case(#[case] str1: String, #[case] str2: String, #[case] expected: String) {
        let actual = Solution::shortest_common_supersequence(str1, str2);
        assert_eq!(actual, expected);
    }
}
