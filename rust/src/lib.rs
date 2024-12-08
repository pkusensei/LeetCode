mod dsu;
mod helper;
mod trie;

use std::cmp::Reverse;

#[allow(unused_imports)]
use helper::*;

pub fn max_two_events(events: &mut [[i32; 3]]) -> i32 {
    events.sort_unstable_by_key(|v| v[0]);
    let n = events.len();
    let mut suffix = Vec::with_capacity(n);
    let mut curr = 0;
    for e in events.iter().rev() {
        curr = curr.max(e[2]);
        suffix.push(curr);
    }
    suffix.reverse();
    let mut res = 0;
    for e in events.iter() {
        let i = events.partition_point(|v| v[0] <= e[1]);
        let temp = e[2] + suffix.get(i).unwrap_or(&0);
        res = res.max(temp);
    }
    res
}

fn with_heap(events: &mut [[i32; 3]]) -> i32 {
    events.sort_unstable_by_key(|v| v[0]);
    let mut heap = std::collections::BinaryHeap::new();
    let mut res = 0;
    let mut temp = 0; // record previous max value
    for e in events.iter() {
        while heap.peek().is_some_and(|&(Reverse(end), _v)| end < e[0]) {
            let (_, val) = heap.pop().unwrap();
            temp = temp.max(val);
        }
        res = res.max(e[2] + temp);
        heap.push((Reverse(e[1]), e[2]));
    }
    res
}

fn greedy(events: &[[i32; 3]]) -> i32 {
    let mut times = Vec::with_capacity(2 * events.len());
    for e in events.iter() {
        times.push([e[0], 1, e[2]]);
        times.push([1 + e[1], 0, e[2]]); // 0 so that sorting put end first
    }
    times.sort_unstable();
    let mut res = 0;
    let mut temp = 0;
    for t in times {
        if t[1] == 1 {
            res = res.max(t[2] + temp);
        } else {
            temp = temp.max(t[2]);
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
        assert_eq!(greedy(&mut [[1, 3, 2], [4, 5, 2], [2, 4, 3]]), 4);
        assert_eq!(greedy(&mut [[1, 3, 2], [4, 5, 2], [1, 5, 5]]), 5);
        assert_eq!(greedy(&mut [[1, 5, 3], [1, 5, 1], [6, 6, 5]]), 8);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
