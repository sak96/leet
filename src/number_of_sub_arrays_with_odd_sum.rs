//! Solution for https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum
//! 1524. Number of Sub-arrays With Odd Sum

impl Solution {
    const MOD: i32 = 10i32.pow(9) + 7;
    pub fn num_of_subarrays(mut arr: Vec<i32>) -> i32 {
        let (mut output, mut odds, mut evens) = (0, 0, 1);
        let mut is_sum_odd = false;
        while let Some(value) = arr.pop() {
            is_sum_odd ^= value & 1 == 1;
            if is_sum_odd {
                odds += 1;
                output += evens
            } else {
                evens += 1;
                output += odds
            }
            output %= Self::MOD
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
    #[case(vec![1,3,5], 4)]
    #[case(vec![2,4,6], 0)]
    #[case(vec![1,2,3,4,5,6,7], 16)]
    #[case(vec![100,100,99, 99], 4)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_of_subarrays(arr);
        assert_eq!(actual, expected);
    }
}
