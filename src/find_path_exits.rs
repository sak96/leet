impl Solution {
    #[inline]
    pub fn find_group_id(groups: &[i32], mut value: i32) -> i32 {
        let mut parent;
        loop {
            parent = groups[value as usize];
            if value == parent {
                break parent;
            }
            value = parent;
        }
    }

    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut groups: Vec<_> = (0..n).collect();
        for edge in edges {
            let x = Self::find_group_id(&groups, edge[0]);
            let y = Self::find_group_id(&groups, edge[1]);
            let src = x.min(y);
            let des = x.max(y);
            groups[des as usize] = src;
        }
        Self::find_group_id(&groups, source) == Self::find_group_id(&groups, destination)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let edges = [[0, 1], [1, 2], [2, 0]];
        let source = 0;
        let destination = 2;
        let output = true;

        let edges = edges.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::valid_path(n, edges, source, destination), output);
    }

    #[test]
    fn case2() {
        let n = 6;
        let edges = [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]];
        let source = 0;
        let destination = 5;
        let output = false;

        let edges = edges.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::valid_path(n, edges, source, destination), output);
    }
}
