//! Solution for https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix
//! 3043. Find the Length of the Longest Common Prefix

impl Solution {
    const POWERS: [i32; 9] = [
        1,
        10,
        100,
        1000,
        1_0000,
        10_0000,
        100_0000,
        1000_0000,
        1_0000_0000,
    ];
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut prefix = std::collections::BTreeSet::new();
        for i in arr1 {
            for j in Self::POWERS.iter() {
                let m = i / j;
                if m == 0 {
                    break;
                }
                prefix.insert(m);
            }
        }
        let mut output = 0;
        for i in arr2 {
            let mut zero = 0;
            for (k, j) in Self::POWERS.iter().enumerate().rev() {
                let m = i / j;
                if m == 0 {
                    zero += 1;
                } else if prefix.contains(&m) {
                    output = output.max(9 - k - zero);
                }
            }
        }
        output as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,10,100], vec![1000], 3)]
    #[case(vec![1,2,3], vec![4,4,4], 0)]
    #[case(vec![13,27,45], vec![21,27,48], 2)]
    #[case(vec![1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000,1,10,100,1000,10000],vec![100000000], 5)]
    fn case(#[case] arr1: Vec<i32>, #[case] arr2: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_common_prefix(arr1, arr2);
        assert_eq!(actual, expected);
    }
}
