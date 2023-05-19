impl Solution {
    #[inline]
    pub fn find_group_id(groups: &[usize], mut value: usize) -> usize {
        let mut parent;
        loop {
            parent = groups[value];
            if value == parent {
                break parent;
            }
            value = parent;
        }
    }

    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let len = graph.len();
        let mut groups: Vec<_> = (0..len).collect();
        for (cur, dest_nodes) in graph.into_iter().enumerate() {
            if let Some(dest_first) = dest_nodes.first() {
                // join all destination to same group.
                let parent_first = Self::find_group_id(&groups, *dest_first as usize);
                let parent_cur = Self::find_group_id(&groups, cur);
                for dest in dest_nodes {
                    let dest_parent = Self::find_group_id(&groups, dest as usize);
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
