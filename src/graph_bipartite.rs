impl Solution {
    pub fn bipartite(
        bipartite: &mut [Option<bool>],
        graph: &mut [Vec<i32>],
        idx: usize,
        state: bool,
    ) -> bool {
        if let Some(b) = bipartite[idx] {
            b == state
        } else {
            bipartite[idx] = Some(state);
            while let Some(node) = graph[idx].pop() {
                if !Self::bipartite(bipartite, graph, node as usize, !state) {
                    return false;
                }
            }
            true
        }
    }

    pub fn is_bipartite(mut graph: Vec<Vec<i32>>) -> bool {
        let len = graph.len();
        let mut bipartite = vec![None; len].into_boxed_slice();
        for idx in 0..len {
            if bipartite[idx].is_some() {
                continue;
            }
            if !Self::bipartite(&mut bipartite, &mut graph, idx, true) {
                return false;
            }
        }

        true
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let graph: Vec<Vec<i32>> = [
            [1, 2, 3].into(),
            [0, 2].into(),
            [0, 1, 3].into(),
            [0, 2].into(),
        ]
        .into();
        let output = false;

        assert_eq!(Solution::is_bipartite(graph), output);
    }

    #[test]
    fn case2() {
        let graph = [[1, 3], [0, 2], [1, 3], [0, 2]];
        let output = true;

        let graph = graph.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::is_bipartite(graph), output);
    }
}
