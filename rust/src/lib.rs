mod helper;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct Twitter {
    tweets: Vec<Tweet>,
    follows: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Default::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let tweet = Tweet {
            author: user_id,
            tweet_id,
        };
        self.tweets.push(tweet);
        self.follows
            .entry(user_id)
            .or_insert(HashSet::from([user_id])); // First tweet
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        if let Some(v) = self.follows.get(&user_id) {
            self.tweets
                .iter()
                .rev()
                .filter_map(|t| {
                    if v.contains(&t.author) {
                        Some(t.tweet_id)
                    } else {
                        None
                    }
                })
                .take(10)
                .collect()
        } else {
            self.follows.insert(user_id, HashSet::from([user_id]));
            vec![]
        }
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows
            .entry(follower_id)
            .or_insert(HashSet::from([follower_id]))
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows
            .entry(follower_id)
            .or_insert(HashSet::from([follower_id]))
            .remove(&followee_id);
    }
}

#[derive(Debug, Clone, Copy)]
struct Tweet {
    author: i32,
    tweet_id: i32,
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5); // User 1 posts a new tweet (id = 5).
        debug_assert_eq!(twitter.get_news_feed(1), [5]); // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
        twitter.follow(1, 2); // User 1 follows user 2.
        twitter.post_tweet(2, 6); // User 2 posts a new tweet (id = 6).
        debug_assert_eq!(twitter.get_news_feed(1), [6, 5]); // User 1's news feed should return a list with 2 tweet ids -> [6, 5]. Tweet id 6 should precede tweet id 5 because it is posted after tweet id 5.
        twitter.unfollow(1, 2); // User 1 unfollows user 2.
        debug_assert_eq!(twitter.get_news_feed(1), [5]); // User 1's news feed should return a list with 1 tweet id -> [5], since user 1 is no longer following user 2.
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
