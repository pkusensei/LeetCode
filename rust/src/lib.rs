mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn median_sliding_window(nums: &[i32], k: i32) -> Vec<f64> {
    let n = k as usize;
    let mut mf = MedianFinder::new(&nums[..n]);
    let mut res = vec![];
    res.push(mf.get_median());
    for (i, &num) in nums.iter().enumerate().skip(n) {
        mf.remove(nums[i - n]);
        mf.add(num);
        res.push(mf.get_median());
    }
    res
}

#[derive(Debug, Clone)]
struct MedianFinder {
    // Apparently BinaryHeap is too slow
    small: BTreeMap<i32, i32>,
    small_count: i32,
    large: BTreeMap<i32, i32>,
    large_count: i32,
}

impl MedianFinder {
    fn new(window: &[i32]) -> Self {
        let n = window.len();
        let mut sorted = window.to_vec();
        sorted.sort_unstable();
        let (small, large, sc, lc) = if n & 1 == 1 {
            let temp = sorted.split_off(n / 2 + 1);
            (sorted, temp, n / 2 + 1, n / 2)
        } else {
            let temp = sorted.split_off(n / 2);
            (sorted, temp, n / 2, n / 2)
        };
        Self {
            small: small.into_iter().fold(BTreeMap::new(), |mut acc, n| {
                *acc.entry(n).or_insert(0) += 1;
                acc
            }),
            small_count: sc as _,
            large: large.into_iter().fold(BTreeMap::new(), |mut acc, n| {
                *acc.entry(n).or_insert(0) += 1;
                acc
            }),
            large_count: lc as _,
        }
    }

    fn get_median(&self) -> f64 {
        if self.small_count > self.large_count {
            f64::from(*self.small.keys().last().unwrap())
        } else {
            let a = *self.small.keys().last().unwrap();
            let b = *self.large.keys().next().unwrap();
            (f64::from(a) + f64::from(b)) / 2.0
        }
    }

    fn add(&mut self, num: i32) {
        if self.large.keys().next().is_some_and(|&n| n <= num) {
            *self.large.entry(num).or_insert(0) += 1;
            self.large_count += 1;
        } else {
            *self.small.entry(num).or_insert(0) += 1;
            self.small_count += 1;
        }
        self.balance();
    }

    fn remove(&mut self, num: i32) {
        if self.small.contains_key(&num) {
            self.small_count -= 1;
            Self::remove_from(&mut self.small, num);
        } else {
            self.large_count -= 1;
            Self::remove_from(&mut self.large, num);
        }
        self.balance();
    }

    fn remove_from(map: &mut BTreeMap<i32, i32>, num: i32) {
        map.entry(num).and_modify(|count| *count -= 1);
        if map.get(&num).is_some_and(|&c| c == 0) {
            map.remove(&num);
        }
    }

    fn balance(&mut self) {
        if self.small_count > self.large_count + 1 {
            let num = *self.small.keys().last().unwrap();
            self.small_count -= 1;
            Self::remove_from(&mut self.small, num);
            self.large_count += 1;
            *self.large.entry(num).or_insert(0) += 1;
        }
        if self.small_count < self.large_count {
            let num = *self.large.keys().next().unwrap();
            self.large_count -= 1;
            Self::remove_from(&mut self.large, num);
            self.small_count += 1;
            *self.small.entry(num).or_insert(0) += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            median_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            [1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000]
        );
        debug_assert_eq!(
            median_sliding_window(&[1, 2, 3, 4, 2, 3, 1, 4, 2], 3),
            [2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000]
        );
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
}
