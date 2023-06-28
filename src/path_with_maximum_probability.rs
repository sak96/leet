//! Solution for https://leetcode.com/problems/path-with-maximum-probability
//! 1514. Path with Maximum Probability

#[derive(PartialEq, PartialOrd)]
struct Number(f64);

impl Eq for Number {}

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
        let mut new_paths = std::collections::BinaryHeap::new();
        let mut prob_map = vec![0.0; n as usize];
        new_paths.push((Number(1.0), start as usize));
        prob_map[start as usize] = 1.0;
        while let Some((Number(p), node)) = new_paths.pop() {
            let prob = prob_map[node];
            if node == end as usize {
                return prob;
            }
            if prob > p {
                continue;
            }
            for (dest, path_prob) in edges.iter().zip(&succ_prob).filter_map(|(e, p)| {
                if e[0] as usize == node {
                    Some((e[1] as usize, p))
                } else if e[1] as usize == node {
                    Some((e[0] as usize, p))
                } else {
                    None
                }
            }) {
                if prob_map[dest] < prob * path_prob {
                    prob_map[dest] = prob * path_prob;
                    new_paths.push((Number(prob * path_prob), dest as usize));
                }
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
