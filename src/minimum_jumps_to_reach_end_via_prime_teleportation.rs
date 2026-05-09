//! Solution for https://leetcode.com/problems/minimum-jumps-to-reach-end-via-prime-teleportation
//! 3629. Minimum Jumps to Reach End via Prime Teleportation

use std::sync::OnceLock;

impl Solution {
    const LEN: usize = 1_000_000;
    pub fn factors() -> &'static Vec<Vec<usize>> {
        static FACTORS: OnceLock<Vec<Vec<usize>>> = OnceLock::new();
        FACTORS.get_or_init(|| {
            let len = Self::LEN + 1;
            let mut factors = vec![vec![]; len + 2];
            if len < 2 {
                return factors;
            }
            for p in std::iter::once(2).chain((3..=len).step_by(2)) {
                if factors[p].is_empty() {
                    for multiple in (p..=len).step_by(p) {
                        factors[multiple].push(p);
                    }
                }
            }
            factors
        })
    }

    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().copied().unwrap() as usize;
        let factors = Self::factors();
        let mut queue = vec![nums.len() - 1];
        let mut next = vec![];
        let mut seen = vec![false; nums.len() + 1];
        seen[nums.len()] = true;
        let mut prime_jumps = vec![vec![]; max + 1];

        for (i, &val) in nums.iter().enumerate() {
            let val = val as usize;
            if factors[val].len() == 1 {
                prime_jumps[val].push(i);
            }
        }

        let mut count = 0;
        while !queue.is_empty() {
            for i in queue.drain(..) {
                if i == 0 {
                    next.truncate(0);
                    break;
                }
                if seen[i] {
                    continue;
                }
                seen[i] = true;
                let num = nums[i];
                for &f in &factors[num as usize] {
                    next.extend(prime_jumps[f].iter());
                }
                next.push(i + 1);
                next.push(i - 1);
            }
            std::mem::swap(&mut queue, &mut next);
            count += 1;
        }
        count - 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1], 0)]
    #[case(vec![1,2,4,6], 2)]
    #[case(vec![2,3,4,7,9],2)]
    #[case(vec![4,6,5,8], 3)]
    #[case(vec![7,5,7], 1)]
    #[case(vec![25,5,7,3,25], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_jumps(nums);
        assert_eq!(actual, expected);
    }
}
