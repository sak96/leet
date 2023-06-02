impl Solution {
    pub fn dfs(id: usize, seen: &mut [bool], neighs: &[Vec<usize>]) -> i32 {
        if seen[id] {
            0
        } else {
            seen[id] = true;
            neighs[id]
                .iter()
                .map(|&i| Self::dfs(i, seen, neighs))
                .sum::<i32>()
                + 1
        }
    }
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut neighs = Vec::with_capacity(bombs.len());
        neighs.push(vec![]);
        for at in 1..bombs.len() {
            let (sub_bombs, rest) = bombs.as_slice().split_at(at);
            let cur_bomb = &rest[0];
            neighs.push(vec![]);
            for (i, bomb) in sub_bombs.iter().enumerate() {
                let distance = ((bomb[0] - cur_bomb[0]) as i64).pow(2)
                    + ((bomb[1] - cur_bomb[1]) as i64).pow(2);
                if distance <= (bomb[2] as i64).pow(2) {
                    neighs[i].push(at);
                }
                if distance <= (cur_bomb[2] as i64).pow(2) {
                    neighs[at].push(i);
                }
            }
        }
        (0..bombs.len())
            .map(|i| Self::dfs(i, &mut vec![false; bombs.len()], &neighs))
            .max()
            .unwrap()
    }
}
pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_builder;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(graph_builder![[2,1,3],[6,1,4]], 2)]
    #[case::leet1(graph_builder![[1,1,5],[10,10,5]], 1)]
    #[case::leet1(graph_builder![[1,1,100000],[100000,100000,1]], 1)]
    #[case::leet1(graph_builder![[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]], 5)]
    fn test(#[case] bombs: Vec<Vec<i32>>, #[case] expected: i32) {
        assert_eq!(Solution::maximum_detonation(bombs), expected);
    }
}
