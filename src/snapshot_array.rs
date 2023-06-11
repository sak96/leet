//! Solution for https://leetcode.com/problems/snapshot-array
pub struct SnapshotArray {
    /// Vec of Vec (snap_id, val)
    map: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        Self {
            map: vec![vec![]; length as usize],
            snap_id: 0,
        }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        let vec = &mut self.map[index as usize];
        if vec.last().map(|&(snap_id, _)| snap_id) == Some(self.snap_id) {
            vec.last_mut().unwrap().1 = val
        } else {
            vec.push((self.snap_id, val));
        }
    }

    pub fn snap(&mut self) -> i32 {
        let snap_id = self.snap_id;
        self.snap_id += 1;
        snap_id
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        debug_assert!(snap_id < self.snap_id);
        let vec = &self.map[index as usize];
        match vec.binary_search_by_key(&snap_id, |&(x, _)| x) {
            Ok(i) => vec.get(i).unwrap().1,
            Err(0) => 0,
            Err(i) => vec.get(i - 1).unwrap().1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn leet1() {
        let mut snapshot_array = SnapshotArray::new(3);
        snapshot_array.set(0, 5);
        assert_eq!(snapshot_array.snap(), 0);
        snapshot_array.set(0, 6);
        assert_eq!(snapshot_array.get(0, 0), 5);
    }
}
