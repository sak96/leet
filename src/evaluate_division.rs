use std::collections::BTreeMap;
impl Solution {
    pub fn search(
        c: usize,
        d: usize,
        map: &BTreeMap<usize, Vec<(usize, f64)>>,
        seen: &mut Vec<usize>,
    ) -> Option<f64> {
        let map_ = map.get(&c)?;
        if c == d {
            return Some(1.0);
        }
        // TODO: avoid cloning
        seen.push(c);
        for (c, out2) in map_.iter() {
            if seen.contains(c) {
                continue;
            }
            if let Some(out) = Self::search(*c, d, map, seen) {
                return Some(out * out2);
            }
        }
        seen.pop();
        None
    }

    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map = BTreeMap::new();
        let mut node_ids = Vec::with_capacity(equations.len() * 2);
        for (mut equation, value) in equations.into_iter().zip(values.into_iter()) {
            let a = equation.pop().expect("equations[i].length == 2");
            let b = equation.pop().expect("equations[i].length == 2");
            let a = if let Some(pos) = node_ids.iter().position(|x| a.eq(x)) {
                pos
            } else {
                node_ids.push(a);
                node_ids.len() - 1
            };
            let b = if let Some(pos) = node_ids.iter().position(|x| b.eq(x)) {
                pos
            } else {
                node_ids.push(b);
                node_ids.len() - 1
            };
            map.entry(b).or_insert_with(Vec::new).push((a, 1.0 / value));
            map.entry(a).or_insert_with(Vec::new).push((b, value));
        }
        let mut answer = Vec::with_capacity(queries.len());
        let mut seen = Vec::with_capacity(map.len());
        for mut query in queries {
            let a = query.pop().expect("queries[i].length == 2");
            let b = query.pop().expect("queries[i].length == 2");
            let (a, b) = if let (Some(a), Some(b)) = (
                node_ids.iter().position(|x| a.eq(x)),
                node_ids.iter().position(|x| b.eq(x)),
            ) {
                (a, b)
            } else {
                answer.push(-1.0);
                continue;
            };
            seen.truncate(0);
            let ans = Self::search(a, b, &map, &mut seen).unwrap_or(-1.0);
            answer.push(ans);
        }
        answer
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        [["a","b"],["b","c"]].to_vec(),
        [2.0,3.0].to_vec(),
        [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]].to_vec(),
        [6.00000,0.50000,-1.00000,1.00000,-1.00000].to_vec(),
    )]
    #[case(
        [["a","b"],["b","c"],["bc","cd"]].to_vec(),
        [1.5,2.5,5.0].to_vec(),
        [["a","c"],["c","b"],["bc","cd"],["cd","bc"]].to_vec(),
        [3.75000,0.40000,5.00000,0.20000].to_vec(),
    )]
    #[case(
        [["a","b"]].to_vec(),
        [0.5].to_vec(),
        [["a","b"],["b","a"],["a","c"],["x","y"]].to_vec(),
        [0.50000,2.00000,-1.00000,-1.00000].to_vec(),
    )]
    #[case(
        [["a","e"],["b","e"]].to_vec(),
        [4.0,3.0].to_vec(),
        [["a","b"],["e","e"],["x","x"]].to_vec(),
        [1.33333,1.00000,-1.00000].to_vec(),
    )]
    #[case(
        [["x1","x2"],["x2","x3"],["x3","x4"],["x4","x5"]].to_vec(),
        [3.0,4.0,5.0,6.0].to_vec(),
        [["x1","x5"],["x5","x2"],["x2","x4"],["x2","x2"],["x2","x9"],["x9","x9"]].to_vec(),
        [360.00000,0.00833,20.00000,1.00000,-1.00000,-1.00000].to_vec(),
    )]
    fn test(
        #[case] equations: Vec<[&str; 2]>,
        #[case] values: Vec<f64>,
        #[case] queries: Vec<[&str; 2]>,
        #[case] expected: Vec<f64>,
    ) {
        let equations = equations
            .into_iter()
            .map(|eq| eq.iter().map(|s| s.to_string()).collect())
            .collect();
        let queries = queries
            .into_iter()
            .map(|eq| eq.iter().map(|s| s.to_string()).collect())
            .collect();
        let output = Solution::calc_equation(equations, values, queries);
        assert_eq!(output.len(), expected.len(), "{output:?} != {expected:?}");
        for (pos, (l, r)) in output.iter().zip(expected.iter()).enumerate() {
            assert_float_eq!(l, r, "{output:?} != {expected:?} at {pos}")
        }
    }
}
