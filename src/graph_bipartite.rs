impl Solution {
    #[inline]
    pub fn find_group_id(groups: &mut [usize], value: usize) -> usize {
        let mut parent = groups[value];
        if value != parent {
            parent = Self::find_group_id(groups, parent);
            groups[value] = parent;
        }
        parent
    }

    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let len = graph.len();
        let mut groups: Vec<_> = (0..len).collect();
        for (cur, dest_nodes) in graph.into_iter().enumerate() {
            if let Some(dest_first) = dest_nodes.first() {
                // join all destination to same group.
                let parent_first = Self::find_group_id(&mut groups, *dest_first as usize);
                let parent_cur = Self::find_group_id(&mut groups, cur);
                for dest in dest_nodes {
                    let dest_parent = Self::find_group_id(&mut groups, dest as usize);
                    if dest_parent == parent_cur {
                        // cur node and destination are in same group
                        return false;
                    }
                    // Use the smallest one as parent
                    groups[dest_parent] = parent_first;
                }
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
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert_eq!(Solution::is_bipartite(graph), false);
    }

    #[test]
    fn case2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(Solution::is_bipartite(graph), true);
    }
}
