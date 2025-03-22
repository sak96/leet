//! Solution for https://leetcode.com/problems/count-the-number-of-complete-components
//! 2685. Count the Number of Complete Components

impl Solution {
    pub fn get_parent(set: &mut Vec<usize>, i: usize) -> usize {
        if set[i] == i {
            i
        } else {
            let j = Self::get_parent(set, set[i]);
            set[i] = j;
            set[i]
        }
    }

    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut set: Vec<_> = (0..n).collect();
        let mut edge_count: Vec<_> = vec![1; n];
        let mut node_count: Vec<_> = vec![0; n];
        for edge in edges {
            let (m, n) = (edge[0] as usize, edge[1] as usize);
            let m_par = Self::get_parent(&mut set, m);
            let n_par = Self::get_parent(&mut set, n);
            set[n_par] = m_par;
            edge_count[m] += 1;
            edge_count[n] += 1;
        }
        for i in 0..n {
            let p = Self::get_parent(&mut set, i);
            node_count[p] += 1;
        }
        for (i, edges) in edge_count.into_iter().enumerate() {
            let p = Self::get_parent(&mut set, i);
            if node_count[p] != edges {
                node_count[p] = 0;
            }
        }
        node_count.iter().filter(|a| **a > 0).count() as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4]], 3)]
    #[case(6, vec![vec![0,1],vec![0,2],vec![1,2],vec![3,4],vec![3,5]], 1)]
    #[case(5, vec![vec![2,1],vec![3,2],vec![4,0],vec![4,2]], 0)]
    fn case(#[case] n: i32, #[case] edges: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_complete_components(n, edges);
        assert_eq!(actual, expected);
    }
}
