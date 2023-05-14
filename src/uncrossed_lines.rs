impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut memory = vec![vec![0; nums2.len()]; nums1.len()];
        for (i, x) in nums1.into_iter().enumerate() {
            for (j, y) in nums2.iter().cloned().enumerate() {
                memory[i][j] = if x == y {
                    if i == 0 || j == 0 {
                        1
                    } else {
                        memory[i - 1][j - 1] + 1
                    }
                } else {
                    match (i, j) {
                        (0, 0) => 0,
                        (0, _) => memory[i][j - 1],
                        (_, 0) => memory[i - 1][j],
                        _ => std::cmp::max(memory[i - 1][j], memory[i][j - 1]),
                    }
                };
            }
        }
        memory.pop().unwrap().pop().unwrap()
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums1 = [2, 5, 1, 2, 5];
        let nums2 = [10, 5, 2, 1, 5, 2];
        let output = Solution::max_uncrossed_lines(nums1.into(), nums2.into());
        assert_eq!(output, 3);
    }

    #[test]
    fn case2() {
        let nums1 = [1, 3, 7, 1, 7, 5];
        let nums2 = [1, 9, 2, 5, 1];
        let output = Solution::max_uncrossed_lines(nums1.into(), nums2.into());
        assert_eq!(output, 2);
    }

    #[test]
    fn case3() {
        let nums1 = [10, 5, 2, 1, 5, 2];
        let nums2 = [2, 5, 1, 2, 5];
        let output = Solution::max_uncrossed_lines(nums1.into(), nums2.into());
        assert_eq!(output, 3);
    }

    #[test]
    fn case4() {
        let nums1 = [1, 9, 2, 5, 1];
        let nums2 = [1, 3, 7, 1, 7, 5];
        let output = Solution::max_uncrossed_lines(nums1.into(), nums2.into());
        assert_eq!(output, 2);
    }

    #[test]
    fn case5() {
        let nums1 = [1, 2, 3, 4, 5, 6];
        let nums2 = [2, 3, 1, 6, 4, 5];
        let output = Solution::max_uncrossed_lines(nums1.into(), nums2.into());
        assert_eq!(output, 4);
    }

    #[test]
    fn case6() {
        let nums1 = [1, 2, 4, 1, 4, 4, 3, 5, 5, 1, 4, 4, 4, 1, 4, 3, 4, 2, 4, 2];
        let nums2 = [2, 4, 1, 1, 3, 5, 2, 1, 5, 1, 2, 3, 3, 2, 1, 4, 1, 2, 5, 5];
        let output = Solution::max_uncrossed_lines(nums1.into(), nums2.into());
        assert_eq!(output, 11);
    }
}
