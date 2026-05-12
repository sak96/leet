//! Solution for https://leetcode.com/problems/minimum-initial-energy-to-finish-tasks
//! 1665. Minimum Initial Energy to Finish Tasks

impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|x| x[0] - x[1]);
        let mut answer = 0;
        let mut energy = 0;
        for t in tasks {
            if energy < t[1] {
                let diff = t[1] - energy;
                answer += diff;
                energy += diff;
            }
            energy -= t[0]
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
    #[case(vec![vec![1,2],vec![2,4],vec![4,8]], 8)]
    #[case(vec![vec![1,3],vec![2,4],vec![10,11],vec![10,12],vec![8,9]], 32)]
    #[case(vec![vec![1,7],vec![2,8],vec![3,9],vec![4,10],vec![5,11],vec![6,12]], 27)]
    fn case(#[case] tasks: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::minimum_effort(tasks);
        assert_eq!(actual, expected);
    }
}
