mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct RecentCounter {
    data: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Default::default()
    }

    fn ping(&mut self, t: i32) -> i32 {
        while self.data.front().is_some_and(|&v| v + 3000 < t) {
            self.data.pop_front();
        }
        self.data.push_back(t);
        self.data.len() as _
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut c = RecentCounter::new();
        debug_assert_eq!(c.ping(1), 1); // requests = [1], range is [-2999,1], return 1
        debug_assert_eq!(c.ping(100), 2); // requests = [1, 100], range is [-2900,100], return 2
        debug_assert_eq!(c.ping(3001), 3); // requests = [1, 100, 3001], range is [1,3001], return 3
        debug_assert_eq!(c.ping(3002), 3); // requests = [1, 100, 3001, 3002], range is [2,3002], return 3
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
