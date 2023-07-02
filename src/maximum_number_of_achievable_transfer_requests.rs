//! Solution for https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests
//! 1601. Maximum Number of Achievable Transfer Requests

impl Solution {
    pub fn check_approved(buildings: &mut [i32], requests: &[Vec<i32>], approved: i32) -> i32 {
        if let Some((request, requests)) = requests.split_first() {
            // without apporval
            let mut max_approval = Self::check_approved(buildings, requests, approved);
            buildings[request[0] as usize] += 1;
            buildings[request[1] as usize] -= 1;
            max_approval =
                max_approval.max(Self::check_approved(buildings, requests, approved + 1));
            buildings[request[0] as usize] -= 1;
            buildings[request[1] as usize] += 1;
            max_approval
        } else if buildings.iter().all(|&x| x == 0) {
            approved
        } else {
            0
        }
    }
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        Self::check_approved(&mut vec![0; n as usize], &requests, 0)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(5, vec![vec![0,1],vec![1,0],vec![0,1],vec![1,2],vec![2,0],vec![3,4]], 5)]
    #[case(3, vec![vec![0,0],vec![1,2],vec![2,1]], 3)]
    #[case(4, vec![vec![0,3],vec![3,1],vec![1,2],vec![2,0]], 4)]
    fn case(#[case] n: i32, #[case] requests: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::maximum_requests(n, requests);
        assert_eq!(actual, expected);
    }
}
