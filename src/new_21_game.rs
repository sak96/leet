impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }
        let mut probability = std::collections::VecDeque::with_capacity(max_pts as usize);
        let mut sum_last_max_pts = 1.0;
        probability.push_front(sum_last_max_pts);
        let mut n_give_k = 0.0;
        for i in 1..=(n.min(k + max_pts)) {
            // i th card is be one of 1..max_pts
            // (equi-likely -> divided by max_pts)
            // sum of each card probability is sum_last_max_pts
            let ith_prob = sum_last_max_pts / max_pts as f64;

            if i < k {
                // i is less than k so it adds to sum
                sum_last_max_pts += ith_prob;
                probability.push_front(ith_prob);
            } else {
                // more than k is drawn and i < n
                // so add this probability
                n_give_k += ith_prob;
            }

            if i >= max_pts && !probability.is_empty() {
                // you cannot draw for i - max_pts cards
                // if it is not added then it is fine.
                sum_last_max_pts -= probability.pop_back().unwrap();
            }
        }

        n_give_k
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;
    #[rstest]
    #[case::leet1(10, 1, 10, 1.0)]
    #[case::leet2(6, 1, 10, 0.6)]
    #[case::leet3(21, 17, 10, 0.73278)]
    #[case::leet3(5710, 5070, 8516, 0.13649)]
    fn case1(#[case] n: i32, #[case] k: i32, #[case] max_pts: i32, #[case] output: f64) {
        assert_eq!(
            (Solution::new21_game(n, k, max_pts) * 1e5).round() / 1e5,
            output
        );
    }
}
