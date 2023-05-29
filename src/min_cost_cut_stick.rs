use std::collections::HashMap;
impl Solution {
    pub fn min_cost_(cut_marks: &[i32], memory: &mut HashMap<(i32, i32), i32>) -> i32 {
        if cut_marks.len() <= 2 {
            return 0;
        }
        // both value will be different : (len > 1) + all cuts are distinct
        let key = (*cut_marks.first().unwrap(), *cut_marks.last().unwrap());
        if let Some(value) = memory.get(&key) {
            return *value;
        }
        let value = (1..(cut_marks.len() - 1))
            .map(|cut_at| {
                Self::min_cost_(&cut_marks[0..=cut_at], memory)
                    + Self::min_cost_(&cut_marks[cut_at..], memory)
            })
            .min()
            .unwrap_or(0)
            + key.1
            - key.0;
        memory.insert(key, value);
        value
    }

    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.sort_unstable();
        Self::min_cost_(
            &std::iter::once(0)
                .chain(cuts)
                .chain(std::iter::once(n))
                .collect::<Vec<_>>(),
            &mut Default::default(),
        )
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(7, vec![1,3,4,5], 16)]
    #[case::leet2(9, vec![5,6,1,4,2], 22)]
    #[case::leet3(2, vec![1], 2)]
    fn test(#[case] n: i32, #[case] cuts: Vec<i32>, #[case] min_cost: i32) {
        assert_eq!(Solution::min_cost(n, cuts), min_cost);
    }
}
