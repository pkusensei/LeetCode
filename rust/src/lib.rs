mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_subarray(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let mut prefix = Vec::with_capacity(1 + n);
    prefix.push(0);
    for &num in nums.iter() {
        prefix.push(i64::from(num) + prefix.last().unwrap_or(&0));
    }
    // increasing monoqueue on prefix sum
    let mut monoqueue = std::collections::VecDeque::new();
    let mut res = 1 + n;
    for (idx, p) in prefix.into_iter().enumerate() {
        // maintain mono-increasing
        while monoqueue.back().is_some_and(|&(_, v)| v >= p) {
            monoqueue.pop_back();
        }
        // the front element i1 satisfies [idx]-[i1]>=k
        // pop i1 out and get distance
        while monoqueue
            .front()
            .is_some_and(|&(_, v)| v + i64::from(k) <= p)
        {
            let Some((i, _)) = monoqueue.pop_front() else {
                break;
            };
            res = res.min(idx - i)
        }
        monoqueue.push_back((idx, p));
    }
    if res < 1 + n {
        res as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(shortest_subarray(&[1], 1), 1);
        debug_assert_eq!(shortest_subarray(&[1, 2], 4), -1);
        debug_assert_eq!(shortest_subarray(&[2, -1, 2], 3), 3);
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
