use std::collections::HashMap;

impl Solution {
    pub fn num_of_minutes_(
        head_id: i32,
        manager_map: &mut HashMap<i32, Vec<i32>>,
        inform_time: &[i32],
    ) -> i32 {
        if let Some(emps) = manager_map.remove(&head_id) {
            let time = inform_time[head_id as usize];
            time + emps
                .into_iter()
                .map(|head_id| Self::num_of_minutes_(head_id, manager_map, inform_time))
                .max()
                .unwrap_or(0)
        } else {
            0
        }
    }
    pub fn num_of_minutes(_n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut manager_map = HashMap::new();
        for (emp_id, mgr_id) in manager.into_iter().enumerate() {
            if mgr_id != -1 {
                manager_map
                    .entry(mgr_id)
                    .or_insert_with(Vec::new)
                    .push(emp_id as i32);
            }
        }
        Self::num_of_minutes_(head_id, &mut manager_map, &inform_time)
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
