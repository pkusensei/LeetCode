mod helper;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn nth_super_ugly_number(n: i32, primes: &[i32]) -> i32 {
    let mut res = 0;
    let mut queue = std::collections::BTreeSet::from([1]);
    for _ in 0..n {
        res = queue.pop_first().unwrap();
        queue.extend(primes.iter().filter_map(|p| p.checked_mul(res)));
    }
    res
}

fn with_pq(n: i32, primes: &[i32]) -> i32 {
    let mut queue: BinaryHeap<_> = primes.iter().map(|&p| (Reverse(p), 0, p)).collect();
    let mut res = Vec::with_capacity(n as usize);
    res.push(1);
    while res.len() < n as usize {
        // `idx` points to `res[idx]`, which is to be multiplied by `prime`
        // for prime in primes, they start with res[0] i.e 1
        let Some((Reverse(value), idx, prime)) = queue.pop() else {
            break;
        };
        if !res.last().is_some_and(|&v| v == value) {
            res.push(value);
        }
        if let Some(n) = res[idx + 1].checked_mul(prime) {
            queue.push((Reverse(n), idx + 1, prime));
        }
    }
    *res.last().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(nth_super_ugly_number(12, &[2, 7, 13, 19]), 32);
        debug_assert_eq!(nth_super_ugly_number(1, &[2, 3, 5]), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            nth_super_ugly_number(
                100000,
                &[
                    7, 19, 29, 37, 41, 47, 53, 59, 61, 79, 83, 89, 101, 103, 109, 127, 131, 137,
                    139, 157, 167, 179, 181, 199, 211, 229, 233, 239, 241, 251
                ]
            ),
            1092889481
        );
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
