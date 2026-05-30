//! Solution for https://leetcode.com/problems/block-placement-queries
//! 3161. Block Placement Queries

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut obs = vec![];
        let mut answer = vec![];
        'outer: for q in queries {
            if q[0] == 1 {
                if let Err(i) = obs.binary_search(&q[1]) {
                    obs.insert(i, q[1]);
                } else {
                    unreachable!()
                }
            } else {
                let mut start = 0;
                for &a in &obs {
                    if a < q[1] {
                        if a - start >= q[2] {
                            answer.push(true);
                            continue 'outer;
                        }
                    } else {
                        break;
                    }
                    start = a;
                }
                answer.push(q[1] - start >= q[2]);
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
    fn case(#[case] queries: Vec<Vec<i32>>, #[case] expected: Vec<bool>) {
        let actual = Solution::get_results(queries);
        assert_eq!(actual, expected);
    }
}
