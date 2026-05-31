//! Solution for https://leetcode.com/problems/destroying-asteroids
//! 2126. Destroying Asteroids

impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        asteroids.sort_unstable();
        for i in asteroids {
            let i = i as i64;
            if mass < i {
                return false;
            }
            mass += i;
        }
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(10, vec![3,9,19,5,21], true)]
    #[case(5, vec![4,9,23,4], false)]
    fn case(#[case] mass: i32, #[case] asteroids: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::asteroids_destroyed(mass, asteroids);
        assert_eq!(actual, expected);
    }
}
