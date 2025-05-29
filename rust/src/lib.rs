mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(n: i32, m: i32) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let [n, m] = [n, m].map(|v| v as usize);
    if SIEVE[n] || SIEVE[m] {
        return -1;
    }
    let mut heap = BinaryHeap::from([(Reverse(n), n)]);
    let mut seen = [false; N];
    seen[n] = true;
    while let Some((Reverse(cost), num)) = heap.pop() {
        if num == m {
            return cost as i32;
        }
        for next in step(num) {
            if !seen[next] {
                seen[next] = true;
                heap.push((Reverse(cost + next), next));
            }
        }
    }
    -1
}

fn step(mut num: usize) -> Vec<usize> {
    let mut ds = vec![];
    while num > 0 {
        ds.push(num % 10);
        num /= 10;
    }
    ds.reverse();
    let n = ds.len();
    let mut res = vec![];
    for i in 0..n {
        if ds[i] < 9 {
            ds[i] += 1;
            check(&ds, &mut res);
            ds[i] -= 1;
        }
        if (i == 0 && ds[i] > 1) || (i > 0 && ds[i] > 0) {
            ds[i] -= 1;
            check(&ds, &mut res);
            ds[i] += 1;
        }
    }
    res
}

fn check(ds: &[usize], res: &mut Vec<usize>) {
    let v = ds.iter().fold(0, |acc, d| acc * 10 + d);
    if !SIEVE[v] {
        res.push(v);
    }
}

const N: usize = 10_000;
const SIEVE: [bool; N] = {
    let mut sieve = [true; N];
    sieve[0] = false;
    sieve[1] = false;
    let mut p = 2;
    while p < N {
        if sieve[p] {
            let mut val = p * p;
            while val < N {
                sieve[val] = false;
                val += p;
            }
        }
        p += 1;
    }
    sieve
};

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(min_operations(10, 12), 85);
    }

    #[test]
    fn test() {}
}
