impl Solution {
    pub fn find(groups: &mut [usize], idx: usize) -> usize {
        let mut parent = groups[idx];
        if idx.ne(&parent) {
            parent = Self::find(groups, parent);
            groups[idx] = parent;
        }
        parent
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut circle_num = is_connected.len();
        let mut groups = (0..is_connected.len()).collect::<Vec<_>>();
        for (i, road) in is_connected.into_iter().enumerate() {
            for (j, is_connected) in road.into_iter().enumerate().take(i) {
                if is_connected == 1 {
                    let group_i = Self::find(&mut groups, i);
                    let group_j = Self::find(&mut groups, j);
                    if group_i != group_j {
                        groups[group_i] = group_j;
                        circle_num -= 1;
                    }
                }
            }
        }
        circle_num as i32
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_builder;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(graph_builder![[1,1,0],[1,1,0],[0,0,1]], 2)]
    #[case::leet2(graph_builder![[1,0,0],[0,1,0],[0,0,1]], 3)]
    pub fn test(#[case] is_connected: Vec<Vec<i32>>, #[case] expected: i32) {
        assert_eq!(Solution::find_circle_num(is_connected), expected);
    }
}
