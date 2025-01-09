mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn num_of_minutes(_n: i32, head_id: i32, manager: &[i32], inform_time: &[i32]) -> i32 {
    let map = manager
        .iter()
        .enumerate()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &v)| {
            acc.entry(v).or_default().push(i);
            acc
        });
    let mut queue = VecDeque::from([(head_id, 0)]);
    let mut res = 0;
    while let Some((curr, time)) = queue.pop_front() {
        res = res.max(time);
        if let Some(v) = map.get(&curr) {
            for &next in v.iter() {
                queue.push_back((next as i32, time + inform_time[curr as usize]));
            }
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
        assert_eq!(num_of_minutes(1, 0, &[-1], &[0]), 0);
        assert_eq!(
            num_of_minutes(6, 2, &[2, 2, -1, 2, 2, 2], &[0, 0, 1, 0, 0, 0]),
            1
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            num_of_minutes(7, 6, &[1, 2, 3, 4, 5, 6, -1], &[0, 6, 5, 4, 3, 2, 1]),
            21
        );
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
