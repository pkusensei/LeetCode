mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::{HashMap, HashSet};
#[derive(Default)]
struct FrequencyTracker {
    num_freq: HashMap<i32, i32>,
    freq_num: HashMap<i32, HashSet<i32>>,
}

impl FrequencyTracker {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, number: i32) {
        let v = self.num_freq.entry(number).or_insert(0);
        let old = *v;
        *v += 1;
        self.freq_num.entry(*v).or_default().insert(number);
        if old > 0 {
            let Some(v) = self.freq_num.get_mut(&old) else {
                unreachable!()
            };
            v.remove(&number);
        }
    }

    fn delete_one(&mut self, number: i32) {
        let Some(v) = self.num_freq.get_mut(&number) else {
            return;
        };
        let old = *v;
        *v -= 1;
        if old == 1 {
            self.num_freq.remove(&number);
        }
        let Some(set) = self.freq_num.get_mut(&old) else {
            unreachable!()
        };
        set.remove(&number);
        if old > 1 {
            self.freq_num.entry(old - 1).or_default().insert(number);
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.freq_num.get(&frequency).is_some_and(|s| !s.is_empty())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
