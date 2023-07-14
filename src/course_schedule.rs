//! Solution for https://leetcode.com/problems/course-schedule
//! 207. Course Schedule

impl Solution {
    pub fn is_cyclic(
        i: usize,
        graph: &[Vec<usize>],
        is_cyclic: &mut [Option<bool>],
        is_visted: &mut [bool],
    ) -> bool {
        if let Some(is_safe) = is_cyclic[i] {
            // from memory
            is_safe
        } else if is_visted[i] {
            // cycle are not allowed
            is_cyclic[i] = Some(false);
            false
        } else {
            is_visted[i] = true;
            // safe = all path must lead to safe nodes.
            let safe = graph[i]
                .iter()
                .all(|&i| Self::is_cyclic(i, graph, is_cyclic, is_visted));
            is_visted[i] = false;
            is_cyclic[i] = Some(safe);
            safe
        }
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![vec![]; num_courses as usize];
        let mut is_cyclic = vec![None; num_courses as usize];
        let mut is_visited = vec![false; num_courses as usize];
        for i in prerequisites {
            graph[i[0] as usize].push(i[1] as usize)
        }
        (0..graph.len()).all(|i| Self::is_cyclic(i, &graph, &mut is_cyclic, &mut is_visited))
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, vec![vec![1,0]], true)]
    #[case(2, vec![vec![1,0],vec![0,1]], false)]
    fn case(
        #[case] num_courses: i32,
        #[case] prerequisites: Vec<Vec<i32>>,
        #[case] expected: bool,
    ) {
        let actual = Solution::can_finish(num_courses, prerequisites);
        assert_eq!(actual, expected);
    }
}
