mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct TweetCounts {
    data: HashMap<String, BTreeMap<i32, i32>>,
}

impl TweetCounts {
    fn new() -> Self {
        Default::default()
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        *self
            .data
            .entry(tweet_name)
            .or_default()
            .entry(time)
            .or_insert(0) += 1;
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        mut start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let mut res = vec![];
        let Some(map) = self.data.get(&tweet_name) else {
            return res;
        };

        fn next(freq: &str, start: i32, end: i32) -> Option<[i32; 2]> {
            if start > end {
                return None;
            }
            let interval = match freq {
                "minute" => 60,
                "hour" => 60 * 60,
                "day" => 24 * 60 * 60,
                _ => return None,
            };
            let end = end.min(start + interval - 1);
            Some([start, end])
        }

        while let Some([start, end]) = next(&freq, start_time, end_time) {
            let curr: i32 = map.range(start..=end).map(|(_k, v)| v).sum();
            res.push(curr);
            start_time = 1 + end;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut tc = TweetCounts::new();
        tc.record_tweet("tweet3".into(), 0); // New tweet "tweet3" at time 0
        tc.record_tweet("tweet3".into(), 60); // New tweet "tweet3" at time 60
        tc.record_tweet("tweet3".into(), 10); // New tweet "tweet3" at time 10
        assert_eq!(
            tc.get_tweet_counts_per_frequency("minute".into(), "tweet3".into(), 0, 59),
            [2]
        ); // return [2]; chunk [0,59] had 2 tweets
        assert_eq!(
            tc.get_tweet_counts_per_frequency("minute".into(), "tweet3".into(), 0, 60),
            [2, 1]
        ); // return [2,1]; chunk [0,59] had 2 tweets, chunk [60,60] had 1 tweet
        tc.record_tweet("tweet3".into(), 120); // New tweet "tweet3" at time 120
        assert_eq!(
            tc.get_tweet_counts_per_frequency("hour".into(), "tweet3".into(), 0, 210),
            [4]
        ); // return [4]; chunk [0,210] had 4 tweets
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
