mod helper;
mod trie;

use core::f64;
use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn kth_smallest_prime_fraction(arr: &[i32], k: i32) -> Vec<i32> {
    let mut heap: BinaryHeap<_> = arr
        .iter()
        .skip(1)
        .map(|&n| (Reverse(Fraction { num: 1, den: n }), 0usize))
        .collect();
    let mut res = [0; 2];
    for _ in 0..k {
        let Some((Reverse(curr), idx)) = heap.pop() else {
            break;
        };
        res = [curr.num, curr.den];
        let Some(&next_num) = arr.get(1 + idx) else {
            continue;
        };
        if next_num >= curr.den {
            continue;
        }
        heap.push((
            Reverse(Fraction {
                num: next_num,
                ..curr
            }),
            1 + idx,
        ));
    }
    res.to_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fraction {
    num: i32,
    den: i32,
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// a/b - c/d == (ad - bc)/bd
impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}

fn with_binary_search(arr: &[i32], k: i32) -> [i32; 2] {
    let n = arr.len();
    let (mut left, mut right) = (0.0, 1.0);
    while left < right {
        let mid = left + (right - left) / 2.0;
        let mut max_fraction = 0.0;
        let mut total_smaller_frac = 0;
        let (mut num_idx, mut den_idx) = (0, 0);
        let mut i2 = 1; // denominator
        for i1 in 0..n - 1 {
            while i2 < n && f64::from(arr[i1]) >= mid * f64::from(arr[i2]) {
                i2 += 1; // this is the counter of bigger fractions
            }
            total_smaller_frac += n - i2; // hence this n-i2
            if i2 == n {
                // with current numerator, every fraction is bigger
                // no need to do another bigger numerator
                break;
            }
            let fraction = f64::from(arr[i1]) / f64::from(arr[i2]);
            if fraction > max_fraction {
                num_idx = i1;
                den_idx = i2;
                max_fraction = fraction;
            }
        }
        match total_smaller_frac.cmp(&(k as usize)) {
            std::cmp::Ordering::Less => right = mid,
            std::cmp::Ordering::Equal => return [arr[num_idx], arr[den_idx]],
            std::cmp::Ordering::Greater => left = mid,
        }
    }
    [0; 2]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_binary_search(&[1, 2, 3, 5], 3), [2, 5]);
        debug_assert_eq!(with_binary_search(&[1, 7], 1), [1, 7]);
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
