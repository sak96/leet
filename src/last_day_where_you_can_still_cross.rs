//! Solution for https://leetcode.com/problems/last-day-where-you-can-still-cross
//! 1970. Last Day Where You Can Still Cross

impl Solution {
    pub fn find(groups: &mut [usize], idx: usize) -> usize {
        let mut parent = groups[idx];
        if idx.ne(&parent) {
            parent = Self::find(groups, parent);
            groups[idx] = parent;
        }
        parent
    }

    pub fn latest_day_to_cross(_row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut map = vec![];
        let mut index_to_map = vec![];
        for (i, value) in cells.iter().enumerate() {
            index_to_map.push(i);
            let (r, c) = (value[0], value[1]);
            let (mut max, mut min) = (c, c);
            for (j, value) in cells[0..i].iter().enumerate() {
                let (r1, c1) = (value[0], value[1]);
                if (r - r1).abs() <= 1 && (c - c1).abs() <= 1 {
                    let parent = Self::find(&mut index_to_map, j);
                    if parent != i {
                        let (min1, max1) = map[parent];
                        index_to_map[parent] = i;
                        min = min.min(min1);
                        max = max.max(max1);
                        if min == 1 && max == col {
                            return i as i32;
                        }
                    }
                }
            }
            map.push((min, max));
        }
        unreachable!("cells.length == row * col AND All the values of cells are unique.");
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 2, vec![vec![1,1],vec![2,1],vec![1,2],vec![2,2]], 2)]
    #[case(2, 2, vec![vec![1,1],vec![1,2],vec![2,1],vec![2,2]], 1)]
    #[case(3, 3, vec![vec![1,2],vec![2,1],vec![3,3],vec![2,2],vec![1,1],vec![1,3],vec![2,3],vec![3,2],vec![3,1]], 3)]
    #[case(2, 6, vec![vec![1,4],vec![1,3],vec![2,1],vec![2,5],vec![2,2],vec![1,5],vec![2,4],vec![1,2],vec![1,6],vec![2,3],vec![2,6],vec![1,1]], 8)]
    fn case(
        #[case] row: i32,
        #[case] col: i32,
        #[case] cells: Vec<Vec<i32>>,
        #[case] expected: i32,
    ) {
        let actual = Solution::latest_day_to_cross(row, col, cells);
        assert_eq!(actual, expected);
    }
}
