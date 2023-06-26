impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let limit = candies.iter().max().unwrap() - extra_candies;
        candies.into_iter().map(|x| x >= limit).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case([2,3,5,1,3],3,[true,true,true,false,true])]
    #[case([4,2,1,1,2],1,[true,false,false,false,false])]
    #[case([12,1,12],10,[true,false,true])]
    fn test(
        #[case] candies: impl AsRef<[i32]>,
        #[case] extra_candies: i32,
        #[case] output: impl AsRef<[bool]>,
    ) {
        assert_eq!(
            Solution::kids_with_candies(candies.as_ref().to_vec(), extra_candies),
            output.as_ref()
        );
    }
}
