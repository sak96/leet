//! Solution for https://leetcode.com/problems/combinations
//! 77. Combinations

impl Solution {
    pub fn combine_(
        start: i32,
        end: i32,
        k: i32,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if k == 0 {
            result.push(combination.to_vec());
        } else if end - start >= k {
            for i in start..end {
                combination.push(i);
                Self::combine_(i + 1, end, k - 1, combination, result);
                combination.pop();
            }
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::combine_(
            1,
            n + 1,
            k,
            &mut Vec::with_capacity(k as usize),
            &mut result,
        );
        result
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, 2, vec![vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]])]
    #[case(1, 1, vec![vec![1]])]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::combine(n, k);
        assert_eq!(actual, expected);
    }
}
