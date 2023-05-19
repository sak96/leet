impl Solution {
    // Special thanks to: https://github.com/vim-scripts/DrawIt
    //
    // Toggle distribution based for pair (x,y) with x < y
    // Based on final sum that is chosen
    //
    //                   [no toggle]
    //   [reduce]            0                [increase]
    //   [x & y ] [reduce y] | [increase x]   [ x & y  ]
    //   +-------+           v               +-----------+
    //   |       |-----------+---------------|           |
    //   |   2   |    1      |      1        |     2     |
    //   +-------+-----------+---------------+-----------+
    //  2       x           x+y     y + limit
    //  ----------------- final sum ---------------------->
    //
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        // max sum index = limit + limit + 1 when is y is limit
        let mut toggle_edges = vec![0; 2 * limit + 2];
        let mut nums = std::collections::VecDeque::from(nums);
        while let (Some(a), Some(b)) = (nums.pop_front(), nums.pop_back()) {
            let x = a.min(b) as usize;
            let y = a.max(b) as usize;
            // add only edges so total is got by cumulative sum
            toggle_edges[2] += 2;
            toggle_edges[x + 1] -= 1;
            toggle_edges[x + y] -= 1;
            toggle_edges[x + y + 1] += 1;
            toggle_edges[y + limit + 1] += 1;
        }

        let mut min_moves = i32::MAX;
        let mut toggles = toggle_edges.drain(..2).sum();
        for toggle_edge in toggle_edges {
            toggles += toggle_edge;
            min_moves = min_moves.min(toggles);
        }
        min_moves
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = [1, 2, 2, 1];
        let limit = 2;
        let output = 2;
        assert_eq!(Solution::min_moves(nums.into(), limit), output)
    }

    #[test]
    fn case2() {
        let nums = [1, 2, 4, 3];
        let limit = 4;
        let output = 1;
        assert_eq!(Solution::min_moves(nums.into(), limit), output)
    }

    #[test]
    fn case3() {
        let nums = [1, 2, 1, 2];
        let limit = 2;
        let output = 0;
        assert_eq!(Solution::min_moves(nums.into(), limit), output)
    }
}
