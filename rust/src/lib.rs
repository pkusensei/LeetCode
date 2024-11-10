mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn largest_component_size(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut dsu = dsu::DSU::new(n);
    let factors: HashMap<i32, Vec<_>> =
        nums.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, &num)| {
                let v = primes(num);
                for f in v {
                    acc.entry(f).or_default().push(i);
                }
                acc
            });
    for v in factors.values() {
        for &idx in v.iter().skip(1) {
            dsu.union(v[0], idx);
        }
    }
    dsu.size.iter().copied().max().unwrap_or(1)
}

fn primes(num: i32) -> HashSet<i32> {
    let mut f = 2;
    while f * f <= num {
        if num % f == 0 {
            return primes(num / f)
                .into_iter()
                .chain(std::iter::once(f))
                .collect();
        }
        f += 1;
    }
    HashSet::from([num])
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_component_size(&[4, 6, 15, 35]), 4);
        debug_assert_eq!(largest_component_size(&[20, 50, 9, 63]), 2);
        debug_assert_eq!(largest_component_size(&[2, 3, 6, 7, 4, 12, 21, 39]), 8);
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
