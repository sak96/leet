//! Solution for https://leetcode.com/problems/closest-prime-numbers-in-range
//! 2523. Closest Prime Numbers in Range

impl Solution {
    pub fn is_prime(i: i32) -> bool {
        let sqrt = (i as f32).sqrt() as i32;
        for j in (3..=sqrt).step_by(2) {
            if i % j == 0 {
                return false;
            }
        }
        true
    }
    pub fn closest_primes(mut left: i32, right: i32) -> Vec<i32> {
        if 2 >= left && 3 <= right {
            // special case with only 1 diff
            return vec![2, 3];
        }
        let mut result = [-1, -1];
        let mut last_prime = None;
        let mut prev_diff = i32::MAX;
        if left % 2 == 0 {
            left = left + 1;
        }
        for i in (left..=right).step_by(2) {
            if Self::is_prime(i) {
                if let Some(last_prime) = last_prime {
                    if i - last_prime < prev_diff {
                        result = [last_prime, i];
                        prev_diff = i - last_prime;
                        if prev_diff == 2 {
                            // cannot get smaller than this.
                            break;
                        }
                    }
                }
                last_prime = Some(i);
            }
        }
        result.to_vec()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(10, 19, vec![11, 13])]
    #[case(4, 6, vec![-1, -1])]
    fn case(#[case] left: i32, #[case] right: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::closest_primes(left, right);
        assert_eq!(actual, expected);
    }
}
