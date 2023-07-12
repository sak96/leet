//! Solution for https://leetcode.com/problems/find-eventual-safe-states
//! 802. Find Eventual Safe States

impl Solution {
    pub fn is_safe(
        i: usize,
        graph: &[Vec<i32>],
        is_safe: &mut [Option<bool>],
        is_visted: &mut [bool],
    ) -> bool {
        if let Some(is_safe) = is_safe[i] {
            // from memory
            is_safe
        } else if is_visted[i] {
            // cycle are not allowed
            is_safe[i] = Some(false);
            false
        } else {
            let mut safe = true;
            is_visted[i] = true;
            for &i in &graph[i] {
                if !Self::is_safe(i as usize, graph, is_safe, is_visted) {
                    // by definition safe node all path should be to safe nodes.
                    safe = false;
                    break;
                }
            }
            is_visted[i] = false;
            is_safe[i] = Some(safe);
            safe
        }
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut is_safe = vec![None; graph.len()];
        let mut is_visited = vec![false; graph.len()];

        (0..graph.len())
            .filter_map(|i| Self::is_safe(i, &graph, &mut is_safe, &mut is_visited).then(|| i as _))
            .collect()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![2,3],vec![5],vec![0],vec![5],vec![],vec![]], vec![2,4,5,6])]
    #[case(vec![vec![1,2,3,4],vec![1,2],vec![3,4],vec![0,4],vec![]], vec![4])]
    fn case(#[case] graph: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let actual = Solution::eventual_safe_nodes(graph);
        assert_eq!(actual, expected);
    }
}
