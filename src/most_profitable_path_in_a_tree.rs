//! Solution for https://leetcode.com/problems/most-profitable-path-in-a-tree
//! 2467. Most Profitable Path in a Tree

impl Solution {
    fn dfs_bob(
        amount: &mut [Option<i32>],
        graph: &Vec<Vec<usize>>,
        node: usize,
        depth: usize,
    ) -> Vec<usize> {
        if node == 0 {
            vec![node]
        } else if let Some(node_cost) = amount[node].take() {
            for &i in graph[node].iter() {
                if amount[i].is_some() {
                    let mut result = Self::dfs_bob(amount, graph, i, depth + 1);
                    if !result.is_empty() {
                        amount[node].replace(node_cost);
                        result.push(node);
                        return result;
                    }
                }
            }
            amount[node].replace(node_cost);
            vec![]
        } else {
            vec![]
        }
    }
    fn dfs_alice(
        amount: &mut [Option<i32>],
        graph: &Vec<Vec<usize>>,
        node: usize,
        bob_path: &mut Vec<usize>,
    ) -> i32 {
        if let Some(node_cost) = amount[node].take() {
            let mut leaf_node = true;
            let mut cost = node_cost;
            let bob_index = bob_path.pop();
            let bob_cost = match bob_index {
                Some(x) if x == node => {
                    cost = cost / 2;
                    None
                }
                Some(x) if amount[x].is_some() => amount[x].replace(0),
                _ => None,
            };
            let mut max_cost = i32::MIN;
            for &i in graph[node].iter() {
                if amount[i].is_some() {
                    leaf_node = false;
                    max_cost = max_cost.max(Self::dfs_alice(amount, graph, i, bob_path));
                }
            }
            if let Some(bob) = bob_index {
                bob_path.push(bob);
                amount[bob] = bob_cost;
            }
            amount[node].replace(node_cost);
            if leaf_node {
                cost
            } else {
                max_cost.saturating_add(cost)
            }
        } else {
            i32::MIN
        }
    }

    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut amount = amount.into_iter().map(|x| Some(x)).collect::<Vec<_>>();
        let mut amount = amount.as_mut_slice();
        let mut graph = vec![vec![]; amount.len()];
        for edge in edges {
            let (x, y) = (edge[0] as usize, edge[1] as usize);
            graph[x].push(y);
            graph[y].push(x);
        }
        let mut bob_path = Self::dfs_bob(&mut amount, &graph, bob as usize, 0);
        Self::dfs_alice(&mut amount, &graph, 0, &mut bob_path)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1],vec![1,2],vec![1,3],vec![3,4]], 3, vec![-2,4,2,-4,6], 6)]
    #[case(vec![vec![0,1]], 1, vec![-7280,2350], -7280)]
    #[case(vec![vec![0,5],vec![1,17],vec![1,8],vec![2,7],vec![2,8],vec![2,18],vec![2,10],vec![3,4],vec![3,10],vec![4,6],vec![4,13],vec![5,8],vec![6,12],vec![8,14],vec![9,12],vec![10,19],vec![11,14],vec![13,16],vec![15,19]], 8, vec![3466,9064,3876,-9160,-7420,9780,3018,-5368,-9666,-5730,-7724,-3752,3588,-3552,5152,6462,-5356,-114,5948,8730], 19700)]
    fn case(
        #[case] edges: Vec<Vec<i32>>,
        #[case] bob: i32,
        #[case] amount: Vec<i32>,
        #[case] expected: i32,
    ) {
        let actual = Solution::most_profitable_path(edges, bob, amount);
        assert_eq!(actual, expected);
    }
}
