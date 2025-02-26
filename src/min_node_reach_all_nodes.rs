impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut node_reachable = vec![false; n as usize];

        for edge in edges {
            node_reachable[edge[1] as usize] = true;
        }

        // NOTE: unique solution requires the solution
        //       to be all node which are not reachable
        node_reachable
            .into_iter()
            .enumerate()
            .filter_map(|(node, reachable)| (!reachable).then_some(node as i32))
            .collect()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]];
        let output = [0, 3];
        let input = input.iter().map(|x| x.to_vec()).collect();
        let output: Vec<_> = output.into();
        assert_eq!(Solution::find_smallest_set_of_vertices(6, input), output);
    }

    #[test]
    fn case2() {
        let input = [[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]];
        let output = [0, 2, 3];
        let input = input.iter().map(|x| x.to_vec()).collect();
        let output: Vec<_> = output.into();
        assert_eq!(Solution::find_smallest_set_of_vertices(5, input), output);
    }
}
