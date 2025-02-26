//! Solution for https://leetcode.com/problems/count-all-possible-routes
//! 1575. Count All Possible Routes

impl Solution {
    const MOD: i32 = 1e9 as i32 + 7;
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let len = locations.len();
        let mut dp = vec![vec![0; fuel as usize + 1]; len];
        dp[finish as usize].iter_mut().for_each(|x| *x = 1);
        for fuel_left in 0..dp[0].len() {
            for i in 0..len {
                for j in 0..len {
                    if i == j {
                        continue;
                    }
                    let fuel_consumed = (locations[i] - locations[j]).unsigned_abs() as usize;
                    if fuel_consumed <= fuel_left {
                        dp[i][fuel_left] += dp[j][fuel_left - fuel_consumed];
                        dp[i][fuel_left] %= Self::MOD;
                    }
                }
            }
        }
        dp[start as usize][fuel as usize]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,6,8,4], 1, 3, 5, 4)]
    #[case(vec![4,3,1], 1, 0, 6, 5)]
    #[case(vec![5,2,1], 0, 2, 3, 0)]
    fn case(
        #[case] locations: Vec<i32>,
        #[case] start: i32,
        #[case] finish: i32,
        #[case] fuel: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::count_routes(locations, start, finish, fuel);
        assert_eq!(actual, expected);
    }
}
