//! Solution for https://leetcode.com/problems/unique-paths-ii
//! 63. Unique Paths II

impl Solution {
    fn unique_paths_with_obstacles_(x: usize, y: usize, obstacle_grid: &mut [Vec<i32>]) -> i32 {
        match obstacle_grid.get_mut(x).and_then(|row| row.get_mut(y)) {
            // obstacle | out of grid
            Some(1) | None => 0,
            // not yet calculated
            Some(0) => {
                let output =
                // right
                Self::unique_paths_with_obstacles_(x + 1, y, obstacle_grid) + 
                // down
                Self::unique_paths_with_obstacles_(x, y + 1, obstacle_grid);
                obstacle_grid[x][y] = output;
                output
            }
            // calculated
            Some(v) => *v,
        }
    }
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let val = obstacle_grid
            .last_mut()
            .expect("1 <= m")
            .last_mut()
            .expect("1 <= n");
        if val == &mut 1 {
            0
        } else {
            *val = -1;
            -Self::unique_paths_with_obstacles_(0, 0, &mut obstacle_grid)
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]], 2)]
    #[case(vec![vec![0,1],vec![0,0]], 1)]
    fn case(#[case] obstacle_grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(actual, expected);
    }
}
