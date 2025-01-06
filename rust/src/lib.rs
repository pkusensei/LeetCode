mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn max_events(events: &mut [[i32; 2]]) -> i32 {
    let n = events.len();
    let max_day = events.iter().map(|e| e[1]).max().unwrap_or(100_000);
    events.sort_unstable();
    let mut res = 0;
    let mut idx = 0;
    let mut heap = BinaryHeap::<Reverse<i32>>::new();
    for d in 1..=max_day {
        // remove completed events
        while heap.peek().is_some_and(|v| v.0 < d) {
            heap.pop();
        }
        // push any event that starts on d
        while idx < n && events[idx][0] == d {
            heap.push(Reverse(events[idx][1]));
            idx += 1;
        }
        // select one event ending on d
        if !heap.is_empty() {
            heap.pop();
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_events(&mut [[1, 2], [2, 3], [3, 4]]), 3);
        assert_eq!(max_events(&mut [[1, 2], [2, 3], [3, 4], [1, 2]]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(max_events(&mut [[1, 2], [1, 2], [3, 3], [1, 5], [1, 5]]), 5);
        assert_eq!(max_events(&mut [[1, 4], [4, 4], [2, 2], [3, 4], [1, 1]]), 4);
    }

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
