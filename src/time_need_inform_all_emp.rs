use std::cmp::Reverse;
impl Solution {
    pub fn num_of_minutes(_n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        let mut manager_map = std::collections::HashMap::new();
        for (emp_id, mgr_id) in manager.into_iter().enumerate() {
            if mgr_id == -1 {
                continue;
            }
            manager_map
                .entry(mgr_id)
                .or_insert_with(Vec::new)
                .push(emp_id as i32);
        }
        heap.push(Reverse((0, head_id)));
        let mut max = 0;
        while let Some(Reverse((mut time, cur_id))) = heap.pop() {
            if let Some(emp) = manager_map.remove(&cur_id) {
                time += inform_time[cur_id as usize];
                max = max.max(time);
                for emp_id in emp {
                    if manager_map.contains_key(&emp_id) {
                        heap.push(Reverse((time, emp_id)));
                    }
                }
            }
        }
        max
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
