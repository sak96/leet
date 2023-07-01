//! Solution for https://leetcode.com/problems/fair-distribution-of-cookies
//! 2305. Fair Distribution of Cookies

impl Solution {
    pub fn assign_cookie(children: &mut [i32], cookies: &[i32]) -> i32 {
        if let Some((cookie, cookies)) = cookies.split_first() {
            (0..children.len())
                .map(|i| {
                    children[i] += cookie;
                    let value = Self::assign_cookie(children, cookies);
                    children[i] -= cookie;
                    value
                })
                .min()
                .expect("2 <= k <= cookies.length")
        } else {
            *children.iter().max().expect("2 <= k <= cookies.length")
        }
    }
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut children = vec![0; k as usize];
        Self::assign_cookie(&mut children, &cookies)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![8,15,10,20,8], 2, 31)]
    #[case(vec![6,1,3,2,2,4,1,2], 3, 7)]
    fn case(#[case] cookies: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::distribute_cookies(cookies, k);
        assert_eq!(actual, expected);
    }
}
