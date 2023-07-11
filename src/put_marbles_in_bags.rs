//! Solution for https://leetcode.com/problems/put-marbles-in-bags
//! 2551. Put Marbles in Bags

impl Solution {
    pub fn put_marbles(mut weights: Vec<i32>, k: i32) -> i64 {
        let mut prev = weights.pop().unwrap();
        for w in weights.iter_mut().rev() {
            let temp = prev;
            prev = *w;
            *w += temp;
        }
        weights.sort_unstable();
        let mut answer = 0i64;
        for (min, max) in weights
            .iter()
            .zip(weights.iter().rev())
            .take(k as usize - 1)
        {
            answer += (max - min) as i64
        }
        answer
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5,1], 2, 4)]
    #[case(vec![1,3], 2, 0)]
    fn case(#[case] weights: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::put_marbles(weights, k);
        assert_eq!(actual, expected);
    }
}
