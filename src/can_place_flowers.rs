//! Solution for https://leetcode.com/problems/can-place-flowers
//! 605. Can Place Flowers

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut flowerbed = flowerbed.iter();
        if n == 0 {
            return true;
        }
        while let Some(flower) = flowerbed.next() {
            if flower == &0 {
                if flowerbed.next() == Some(&1) {
                    flowerbed.next();
                } else {
                    n -= 1;
                    if n == 0 {
                        return true;
                    }
                }
            } else {
                flowerbed.next();
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,0,0,1], 1, true)]
    #[case(vec![1,0,0,0,1], 2, false)]
    fn case(#[case] flowerbed: Vec<i32>, #[case] n: i32, #[case] expected: bool) {
        let actual = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(actual, expected);
    }
}
