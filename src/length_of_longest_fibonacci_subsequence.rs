//! Solution for https://leetcode.com/problems/length-of-longest-fibonacci-subsequence
//! 873. Length of Longest Fibonacci Subsequence

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut dp = std::collections::BTreeMap::new();
        let set: std::collections::BTreeSet<i32> = arr.iter().cloned().collect();
        let mut length = 0;
        let arr = arr.as_slice();
        for i in 0..arr.len() {
            let (prev, rest) = arr.split_at(i);
            let z = rest[0];
            for &y in prev {
                let x = z - y;
                if x < y && set.contains(&x) {
                    let len = dp.get(&(y, x)).unwrap_or(&2) + 1;
                    length = length.max(len);
                    dp.insert((z, y), len);
                }
            }
        }
        length
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,5,6,7,8], 5)]
    #[case(vec![1,3,7,11,12,14,18], 3)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::len_longest_fib_subseq(arr);
        assert_eq!(actual, expected);
    }
}
