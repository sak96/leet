use std::collections::VecDeque;
impl Solution {
    const NEIGHBOUR: [[isize; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    #[inline]
    fn overflow_add_signed(u: usize, i: isize) -> Option<usize> {
        (u != 0 || i >= 0).then(|| (u as isize + i) as usize)
    }

    fn mark_island_as_2(
        grid: &mut [Vec<i32>],
        row: usize,
        col: usize,
        borders: &mut VecDeque<(usize, usize)>,
    ) {
        if grid[row][col] != 1 {
            return;
        }
        grid[row][col] = 2;
        let mut is_border = false;
        for n in Self::NEIGHBOUR {
            let r = if let Some(r) = Self::overflow_add_signed(row, n[0]) {
                r
            } else {
                continue;
            };
            let c = if let Some(c) = Self::overflow_add_signed(col, n[1]) {
                c
            } else {
                continue;
            };
            match grid.get(r).map(|x| x.get(c)).flatten() {
                Some(&1) => Self::mark_island_as_2(grid, r, c, borders),
                Some(&0) => is_border = true,
                _ => {}
            }
        }
        if is_border {
            borders.push_front((row, col));
        }
    }

    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        // get first cell of island
        let (row, col) = grid
            .iter()
            .enumerate()
            .flat_map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .find_map(|(j, y)| (y == &1).then(|| (i, j)))
            })
            .next()
            .expect("There are exactly two islands in grid");
        let mut borders = VecDeque::new();
        Self::mark_island_as_2(&mut grid, row, col, &mut borders);

        // do breadth first search for other island
        let mut step = 0;
        'outer: loop {
            let len = borders.len();
            for _ in 0..len {
                let (row, col) = borders.pop_front().expect("len = borders.len()");
                for n in Self::NEIGHBOUR {
                    let r = if let Some(r) = Self::overflow_add_signed(row, n[0]) {
                        r
                    } else {
                        continue;
                    };
                    let c = if let Some(c) = Self::overflow_add_signed(col, n[1]) {
                        c
                    } else {
                        continue;
                    };
                    match grid.get(r).map(|x| x.get(c)).flatten() {
                        Some(&1) => break 'outer step,
                        Some(&0) => {
                            grid[r][c] = 2;
                            borders.push_back((r, c));
                        }
                        _ => {}
                    }
                }
            }
            step += 1;
        }
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_builder;
    use rstest::rstest;

    #[rstest]
    #[case(graph_builder![[0,1],[1,0]],1)]
    #[case(graph_builder![[0,1,0],[0,0,0],[0,0,1]],2)]
    #[case(graph_builder![[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]],1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] output: i32) {
        assert_eq!(Solution::shortest_bridge(grid), output);
    }
}
