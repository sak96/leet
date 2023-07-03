impl Solution {
    const NEIGHBOUR: &'static [[usize; 2]] = &[
        [usize::MAX, usize::MAX],
        [usize::MAX, 0],
        [usize::MAX, 1],
        [0, usize::MAX],
        [0, 1],
        [1, usize::MAX],
        [1, 0],
        [1, 1],
    ];
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let last = grid.len() - 1;
        if grid[0][0] != 0 || grid[last][last] != 0 {
            return -1;
        }
        if last == 0 {
            return 1;
        }

        let mut step = 1;
        let mut borders = std::collections::VecDeque::new();
        borders.push_front((0usize, 0usize));
        while !borders.is_empty() {
            let len = borders.len();
            for _ in 0..len {
                let (row, col) = borders.pop_front().expect("len = borders.len()");
                for n in Self::NEIGHBOUR {
                    let r = row.wrapping_add(n[0]);
                    let c = col.wrapping_add(n[1]);
                    if r == last && c == last {
                        return step + 1;
                    }
                    if matches!(grid.get(r).map(|x| x.get(c)).flatten(), Some(&0)) {
                        grid[r][c] = 1;
                        borders.push_back((r, c));
                    }
                }
            }
            step += 1;
        }
        -1
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![vec![0,1],vec![1,0]], 2)]
    #[case::leet2(vec![vec![0,0,0],vec![1,1,0],vec![1,1,0]], 4)]
    #[case::leet_neg1(vec![vec![1,0,0],vec![1,1,0],vec![1,1,0]], -1)]
    #[case::my_neg1(vec![vec![1,1],vec![1,0]], -1)]
    #[case::my_neg2(vec![vec![0,1],vec![1,1]], -1)]
    #[case::my_neg3(vec![vec![1,1],vec![1,1]], -1)]
    #[case::my_neg3(vec![vec![0]], 1)]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] output: i32) {
        assert_eq!(Solution::shortest_path_binary_matrix(grid), output);
    }
}
