//! Solution for https://leetcode.com/problems/path-with-maximum-probability
//! 1514. Path with Maximum Probability

#[derive(PartialEq, PartialOrd)]
struct Number(f64);

impl Eq for Number {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let mut new_paths = std::collections::BinaryHeap::new();
        let mut visited = vec![false; n];
        let mut adj_prob = vec![vec![]; n];
        for (path, path_prob) in edges.iter().zip(succ_prob) {
            adj_prob[path[0] as usize].push((path[1] as usize, path_prob));
            adj_prob[path[1] as usize].push((path[0] as usize, path_prob));
        }
        new_paths.push((Number(1.0), start as usize));
        while let Some((Number(prob), node)) = new_paths.pop() {
            if node == end as usize {
                return prob;
            }
            if visited[node] {
                continue; // already seen
            }
            visited[node] = true;
            for (dest, path_prob) in &adj_prob[node] {
                new_paths.push((Number(prob * path_prob), { *dest }));
            }
        }
        0.0
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![vec![0,1],vec![1,2],vec![0,2]], vec![0.5,0.5,0.2], 0, 2, 0.25000)]
    #[case(3, vec![vec![0,1],vec![1,2],vec![0,2]], vec![0.5,0.5,0.3], 0, 2, 0.30000)]
    #[case(3, vec![vec![0,1]], vec![0.5], 0, 2, 0.00000)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] succ_prob: Vec<f64>,
        #[case] start: i32,
        #[case] end: i32,
        #[case] expected: f64,
    ) {
        let actual = Solution::max_probability(n, edges, succ_prob, start, end);
        assert_eq!(actual, expected);
    }
}
