impl Solution {
    pub fn slope_eq((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
        if y1 == 0 {
            return y2 == 0;
        }
        // below is expression x1/y1 == x2/y2 by multiplying each side with y1 * y2
        x1 * y2 == x2 * y1
    }
    pub fn check_straight_line(mut coordinates: Vec<Vec<i32>>) -> bool {
        let first = coordinates.pop().unwrap();
        let second = coordinates.pop().unwrap();
        let slope = (first[0] - second[0], first[1] - second[1]);
        while let Some(second) = coordinates.pop() {
            if !Self::slope_eq((first[0] - second[0], first[1] - second[1]), slope) {
                return false;
            }
        }
        true
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5],vec![5,6],vec![6,7]], true)]
    #[case::leet2(vec![vec![1,1],vec![2,2],vec![3,4],vec![4,5],vec![5,6],vec![7,7]], false)]
    #[case::leet3(vec![vec![1,2],vec![2,3],vec![3,5]], false)]
    fn test(#[case] coordinates: Vec<Vec<i32>>, #[case] straight: bool) {
        assert_eq!(Solution::check_straight_line(coordinates), straight);
    }
}
