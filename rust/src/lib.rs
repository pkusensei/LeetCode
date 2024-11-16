mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap};

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct TimeMap {
    data: HashMap<String, BTreeMap<i32, String>>,
}

impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.data
            .get(&key)
            .and_then(|m| {
                m.range(..=timestamp)
                    .next_back()
                    .map(|(_, v)| v.to_string())
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut tm = TimeMap::new();
        tm.set("foo".into(), "bar".into(), 1); // store the key "foo" and value "bar" along with timestamp = 1.
        debug_assert_eq!(tm.get("foo".into(), 1), "bar"); // return "bar"
        debug_assert_eq!(tm.get("foo".into(), 3), "bar"); // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
        tm.set("foo".into(), "bar2".into(), 4); // store the key "foo" and value "bar2" along with timestamp = 4.
        debug_assert_eq!(tm.get("foo".into(), 4), "bar2"); // return "bar2"
        debug_assert_eq!(tm.get("foo".into(), 5), "bar2"); // return "bar2"
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
