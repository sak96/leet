//! Solution for https://leetcode.com/problems/length-of-longest-fibonacci-subsequence
//! 873. Length of Longest Fibonacci Subsequence

use std::cmp::Reverse;

impl Solution {
    pub fn len_longest_fib_subseq(mut arr: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::<Reverse<(i32, i32, i32)>>::new();
        arr.reverse();
        let mut done = vec![];
        let mut length = 0;

        while let Some(n) = arr.pop() {
            for &i in &done {
                heap.push(Reverse((i + n, n, 2)));
            }
            while let Some(Reverse((c, b, l))) = heap.pop() {
                match c.cmp(&n) {
                    std::cmp::Ordering::Less => {
                        length = length.max(l);
                    }
                    std::cmp::Ordering::Equal => {
                        length = length.max(l + 1);
                        heap.push(Reverse((c + b, c, l + 1)));
                    }
                    std::cmp::Ordering::Greater => {
                        heap.push(Reverse((c, b, l)));
                        break;
                    }
                }
            }
            done.push(n);
        }
        while let Some(Reverse((_, _, l))) = heap.pop() {
            length = length.max(l);
        }
        if length > 2 {
            length
        } else {
            0
        }
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
