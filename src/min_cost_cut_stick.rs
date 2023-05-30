impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.push(n);
        cuts.push(0);
        cuts.sort_unstable();
        let len = cuts.len();

        // memory[x][y] => cost of cuts for stick 0..[x...y]..n
        let mut memory = vec![vec![0; len]; len];

        for peice in 2..len {
            // join piece number of cuts together
            for left in 0..len - peice {
                let right = left + peice;
                memory[left][right] = (left + 1..right)
                    .map(|mid| {
                        // cost of joining (left..mid + mid .. right) pieces (dp) + length of stick
                        memory[left][mid] + memory[mid][right] + cuts[right] - cuts[left]
                    })
                    .min()
                    .unwrap();
            }
        }
        // [0..n]
        memory[0][len - 1]
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(7, vec![1,3,4,5], 16)]
    #[case::leet2(9, vec![5,6,1,4,2], 22)]
    #[case::leet3(2, vec![1], 2)]
    fn test(#[case] n: i32, #[case] cuts: Vec<i32>, #[case] min_cost: i32) {
        assert_eq!(Solution::min_cost(n, cuts), min_cost);
    }
}
