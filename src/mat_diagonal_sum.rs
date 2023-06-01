impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let len = mat.len();
        for (i, row) in mat.into_iter().enumerate() {
            let j = len - i - 1;
            sum += row[i];
            if i != j {
                sum += row[j];
            }
        }
        sum
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[1, 2, 3].into(), [4, 5, 6].into(), [7, 8, 9].into()].into();
        assert_eq!(Solution::diagonal_sum(mat), 25);
    }

    #[test]
    fn case2() {
        let mat = [
            [1, 1, 1, 1].into(),
            [1, 1, 1, 1].into(),
            [1, 1, 1, 1].into(),
            [1, 1, 1, 1].into(),
        ]
        .into();
        assert_eq!(Solution::diagonal_sum(mat), 8);
    }

    #[test]
    fn case3() {
        let mat = [[5].into()].into();
        assert_eq!(Solution::diagonal_sum(mat), 5);
    }
}
