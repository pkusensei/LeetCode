mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

#[derive(Default)]
struct CountIntervals {
    data: BTreeMap<i32, i32>,
    count: i32,
}

impl CountIntervals {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, mut left: i32, mut right: i32) {
        while let Some((&k, &v)) = self.data.range(left..).next() {
            if right < k {
                break;
            }
            right = right.max(v);
            self.data.remove(&k);
            self.count -= v - k + 1;
        }
        while let Some((&k, &v)) = self.data.range(..right).next_back() {
            if v < left {
                break;
            }
            left = left.min(k);
            right = right.max(v);
            self.data.remove(&k);
            self.count -= v - k + 1;
        }
        self.data.insert(left, right);
        self.count += right - left + 1;
    }

    fn count(&self) -> i32 {
        self.count // O(n) sum TLE's
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
    fn basics() {
        let mut countIntervals = CountIntervals::new();
        countIntervals.add(2, 3); // add [2, 3] to the set of intervals.
        countIntervals.add(7, 10); // add [7, 10] to the set of intervals.
        assert_eq!(countIntervals.count(), 6);
        // the integers 2 and 3 are present in the interval [2, 3].
        // the integers 7, 8, 9, and 10 are present in the interval [7, 10].
        countIntervals.add(5, 8); // add [5, 8] to the set of intervals.
        assert_eq!(countIntervals.count(), 8);
    }

    #[test]
    fn test() {}
}
