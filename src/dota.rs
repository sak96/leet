impl Solution {
    pub fn predict_party_victory(mut senate: String) -> String {
        let bytes = unsafe { senate.as_mut_vec() };
        let mut cur_voters = 0;
        let mut cur_element = bytes[0];
        let mut end_index = bytes.len();
        loop {
            let mut swap = false;
            let mut insert_index = 0;
            for seek_index in 0..end_index {
                let mut insert = false;
                if bytes[seek_index] == cur_element {
                    insert = true;
                } else if cur_voters == 0 {
                    insert = true;
                    swap = true;
                    cur_element = bytes[seek_index];
                } else {
                    cur_voters -= 1;
                }
                if insert {
                    bytes[insert_index] = cur_element;
                    insert_index += 1;
                    cur_voters += 1;
                }
            }
            if !swap {
                break match cur_element {
                    b'R' => "Radiant",
                    b'D' => "Dire",
                    e => unreachable!("{}", e),
                }
                .into();
            }
            end_index = insert_index;
        }
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::predict_party_victory("RD".into()), "Radiant");
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::predict_party_victory("RDD".into()), "Dire");
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::predict_party_victory("RRDDD".into()), "Radiant");
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::predict_party_victory("DRRDRDRDRDDRDRDR".into()),
            "Radiant"
        );
    }
}
