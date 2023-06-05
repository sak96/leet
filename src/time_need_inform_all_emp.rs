impl Solution {
    pub fn num_of_minutes_(
        emp_id: usize,
        manager: &[i32],
        inform_time: &[i32],
        memory: &mut [Option<i32>],
    ) -> i32 {
        if let Some(time) = memory[emp_id] {
            time
        } else {
            let mgr_id = manager[emp_id] as usize;
            let time =
                inform_time[mgr_id] + Self::num_of_minutes_(mgr_id, manager, inform_time, memory);
            memory[emp_id] = Some(time);
            time
        }
    }
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut memory = vec![None; n as usize];
        memory[head_id as usize] = Some(0);
        (0..memory.len())
            .into_iter()
            .map(|emp_id| Self::num_of_minutes_(emp_id, &manager, &inform_time, &mut memory))
            .max()
            .expect("1 <= n")
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::leet1(1, 0, vec![-1], vec![0], 0)]
    #[case::leet2(6, 2, vec![2,2,-1,2,2,2], vec![0,0,1,0,0,0], 1)]
    #[case::leet2(8, 0, vec![-1,5,0,6,7,0,0,0], vec![89,0,0,0,0,523,241,519], 612)]
    pub fn test(
        #[case] n: i32,
        #[case] head_id: i32,
        #[case] manager: Vec<i32>,
        #[case] inform_time: Vec<i32>,
        #[case] expected: i32,
    ) {
        assert_eq!(
            Solution::num_of_minutes(n, head_id, manager, inform_time),
            expected
        );
    }
}
