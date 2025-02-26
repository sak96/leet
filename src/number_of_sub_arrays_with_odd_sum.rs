//! Solution for https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum
//! 1524. Number of Sub-arrays With Odd Sum

impl Solution {
    const MOD: i32 = 10i32.pow(9) + 7;
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut is_change = false;
        let mut odds = 0;
        let arr: Vec<_> = arr
            .into_iter()
            .map(|v| {
                let is_odd = v % 2 == 1;
                is_change ^= is_odd;
                if is_change {
                    odds += 1;
                }
                is_odd
            })
            .collect();
        let mut evens = arr.len() as i32 - odds;
        let mut sum = 0;
        let mut slice = arr.as_slice();
        while let Some((&is_odd, rest)) = slice.split_first() {
            sum += odds;
            if is_odd {
                odds -= 1;
                std::mem::swap(&mut odds, &mut evens);
            } else {
                evens -= 1;
            }
            sum %= Self::MOD;
            slice = rest;
        }
        sum
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
