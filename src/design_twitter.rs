use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
// Twitter Storage
pub struct Twitter {
    /// tweets stack
    tweets: Vec<(i32, i32)>,
    /// follower -> followee
    /// add self as follower
    follower: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    /// Use default implementation
    pub fn new() -> Self {
        Self::default()
    }

    /// Add post to stak
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id))
    }

    /// Get Top 10 Tweets from user or his followee
    ///
    /// - Get all followees for user. (This includes user!)
    /// - Iterate through stack
    /// - Filter tweets belonging to followees
    /// - Take 10 and put it as vector
    #[must_use]
    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let followees = self
            .follower
            .entry(user_id)
            .or_insert_with_key(Self::get_default_followee);
        self.tweets
            .iter()
            .rev()
            .filter_map(|(u_id, t_id)| followees.contains(u_id).then(|| *t_id))
            .take(10)
            .collect()
    }

    /// Get default followee of user. (includes user!).
    #[inline]
    fn get_default_followee(user_id: &i32) -> HashSet<i32> {
        std::iter::once(*user_id).collect()
    }

    /// Add follower -> followee relation
    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follower
            .entry(follower_id)
            .or_insert_with_key(Self::get_default_followee)
            .insert(followee_id);
    }

    /// Remove follower -> follower if follower is not followee
    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            self.follower
                .get_mut(&follower_id)
                .map(|f| f.remove(&followee_id));
        }
    }
}

/// https://leetcode.com/problems/design-twitter/description/
#[cfg(test)]
mod tests {

    use super::*;

    /// Case provided by leet code
    #[test]
    fn case1() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), [5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), [6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), [5]);
    }

    /// Case to valid user cannot unfollow self
    #[test]
    fn case2() {
        let user_id = 1;
        let post_id = 10;
        let mut twitter = Twitter::new();
        twitter.post_tweet(user_id, post_id);
        twitter.unfollow(user_id, user_id);
        assert_eq!(twitter.get_news_feed(user_id), [post_id]);
    }
}
