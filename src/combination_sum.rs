//! Solution for https://leetcode.com/problems/combination-sum

use std::collections::HashMap;
impl Solution {
    pub fn combination_sum_(
        candidates: &[i32],
        start: usize,
        target: i32,
        memory: &mut HashMap<(usize, i32), Vec<Vec<i32>>>,
    ) -> Vec<Vec<i32>> {
        let mut output = vec![];
        let key = (start, target);
        if candidates.is_empty() {
            return output;
        } else if let Some(vec) = memory.get(&key) {
            return vec.clone();
        } else if target == candidates[0] {
            output.push(vec![target]);
        } else {
            let including = match candidates.binary_search(&(target - candidates[0])) {
                Ok(i) => {
                    Self::combination_sum_(&candidates[..=i], start, target - candidates[0], memory)
                }
                Err(i) => {
                    Self::combination_sum_(&candidates[..i], start, target - candidates[0], memory)
                }
            };
            for mut vec in including {
                vec.push(candidates[0]);
                output.push(vec);
            }
            output.extend(Self::combination_sum_(
                &candidates[1..],
                start + 1,
                target,
                memory,
            ))
        }
        memory.insert(key, output.clone());
        output
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        Self::combination_sum_(&candidates, 0, target, &mut HashMap::new())
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    #[rstest]
    #[case::leet1(vec![2,3,6,7],7,vec![vec![2,2,3],vec![7]])]
    #[case::leet2(vec![2,3,5],8,vec![vec![2,2,2,2],vec![2,3,3],vec![3,5]])]
    #[case::leet2(vec![2],1,vec![])]
    fn test(#[case] candidates: Vec<i32>, #[case] target: i32, #[case] expected: Vec<Vec<i32>>) {
        let mut output = Solution::combination_sum(candidates, target);
        for el in &mut output {
            el.sort_unstable();
        }
        output.sort_unstable();
        assert_eq!(output, expected);
    }
}
