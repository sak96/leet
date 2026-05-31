//! Solution for https://leetcode.com/problems/block-placement-queries
//! 3161. Block Placement Queries

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut obs = vec![i32::MAX];
        let mut gaps = vec![i32::MAX];
        let mut answer = vec![];
        for q in queries {
            let idx = obs.binary_search(&q[1]).unwrap_or_else(|x| x);
            if q[0] == 1 {
                obs.insert(idx, q[1]);
                let existing_gap = gaps[idx];
                if idx == 0 {
                    gaps.insert(idx, q[1]);
                } else {
                    let prev_gap = gaps[idx - 1].max(q[1] - obs[idx - 1]);
                    gaps.insert(idx, prev_gap);
                }
                let mut start = q[1];
                let mut prev_gap = gaps[idx];
                for (i, ob) in obs.iter().enumerate().skip(idx + 1) {
                    let gap = ob - start;
                    gaps[i] = prev_gap.max(gap);
                    prev_gap = gaps[i];
                    start = *ob;
                    if gap >= existing_gap {
                        break;
                    }
                }
            } else {
                if idx == 0 {
                    answer.push(q[1] >= q[2]);
                } else {
                    let gap = gaps[idx - 1];
                    let ob = obs[idx - 1];
                    answer.push((q[1] - ob).max(gap) >= q[2]);
                }
            }
        }
        answer
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![2,3,3],vec![2,3,1],vec![2,2,2]], vec![false,true,true])]
    #[case(vec![vec![1,7],vec![2,7,6],vec![1,2],vec![2,7,5],vec![2,7,6]], vec![true,true,false])]
    #[case(include!("a.txt"), vec![])]
    fn case(#[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<bool>) {
        let actual = Solution::get_results(queries);
        assert_eq!(actual, expected);
    }
}
