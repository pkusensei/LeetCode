mod dsu;
mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn minimized_maximum(n: i32, quantities: &[i32]) -> i32 {
    let mut left = 1;
    let mut right = *quantities.iter().max().unwrap();
    while left < right {
        let mid = left + (right - left) / 2;
        if count(mid, quantities) > n {
            left = 1 + mid;
        } else {
            right = mid;
        }
    }
    left
}

fn count(mid: i32, nums: &[i32]) -> i32 {
    nums.iter().map(|v| v / mid + i32::from(v % mid > 0)).sum()
}

fn with_pq(n: i32, nums: &[i32]) -> i32 {
    #[derive(Debug, Clone, Copy)]
    struct State {
        num: i32,
        count: i32,
    }

    impl PartialEq for State {
        fn eq(&self, other: &Self) -> bool {
            self.num * other.count == other.num * self.count
        }
    }

    impl Eq for State {}

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (self.num * other.count).cmp(&(self.count * other.num))
        }
    }

    let mut pq: BinaryHeap<_> = nums.iter().map(|&v| State { num: v, count: 1 }).collect();
    let m = nums.len();
    for _ in 0..n - m as i32 {
        if let Some(mut v) = pq.peek_mut() {
            v.count += 1;
        }
    }
    let Some(State { num, count }) = pq.pop() else {
        return -1;
    };
    num / count + i32::from(num % count > 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_pq(7, &[11, 3]), 3);
        debug_assert_eq!(with_pq(7, &[15, 10, 10]), 5);
        debug_assert_eq!(with_pq(1, &[100000]), 100000);
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
