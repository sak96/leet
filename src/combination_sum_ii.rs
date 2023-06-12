//! Solution for https://leetcode.com/problems/combination-sum-ii
use std::collections::HashMap;
impl Solution {
    pub fn combination_sum2_(
        candidates: &[i32],
        start: usize,
        target: i32,
        memory: &mut HashMap<(usize, i32), Vec<Vec<i32>>>,
    ) -> Vec<Vec<i32>> {
        let mut output = vec![];
        let key = (start, target);
        if candidates.is_empty() || candidates[0] > target {
            return output;
        } else if let Some(vec) = memory.get(&key) {
            return vec.clone();
        } else if target == candidates[0] {
            output.push(vec![target]);
        } else {
            let candidate = candidates[0];
            let next_pos = candidates
                .iter()
                .position(|x| (candidate).lt(x))
                .unwrap_or(candidates.len());
            let start = start + next_pos;
            let candidates = &candidates[next_pos..];
            for i in 0..=next_pos {
                let target = target - i as i32 * candidate;
                let include = vec![candidate; i];
                if target <= 0 {
                    if target == 0 {
                        output.push(include);
                    }
                    break;
                }
                let including = match candidates.binary_search(&target) {
                    Ok(i) => Self::combination_sum2_(&candidates[..=i], start, target, memory),
                    Err(0) => vec![],
                    Err(i) => Self::combination_sum2_(&candidates[..i], start, target, memory),
                };
                for mut vec in including {
                    vec.extend(include.iter().cloned());
                    output.push(vec);
                }
            }
        }
        memory.insert(key, output.clone());
        output
    }
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        Self::combination_sum2_(&candidates, 0, target, &mut HashMap::new())
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_builder;

    use rstest::rstest;
    #[rstest]
    #[case::leet1(vec![10,1,2,7,6,1,5],8,graph_builder![
        [1,1,6],
        [1,2,5],
        [1,7],
        [2,6]
    ])]
    #[case::leet2(vec![2,5,2,1,2],5,graph_builder![
        [1,2,2],
        [5]
    ])]
    #[case::leet3(vec![4,4,2,1,4,2,2,1,3],6,graph_builder![[1,1,2,2],[1,1,4],[1,2,3],[2,2,2],[2,4]])]
    fn test(#[case] candidates: Vec<i32>, #[case] target: i32, #[case] expected: Vec<Vec<i32>>) {
        let mut output = Solution::combination_sum2(candidates, target);
        for el in &mut output {
            el.sort_unstable();
        }
        output.sort_unstable();
        assert_eq!(output, expected);
    }
}
