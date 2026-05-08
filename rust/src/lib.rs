mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{
    collections::{HashMap, VecDeque},
    sync::LazyLock,
};

#[allow(unused_imports)]
use helper::*;

pub fn min_jumps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut prime_indices = HashMap::<_, Vec<_>>::new();
    for (idx, &num) in nums.iter().enumerate() {
        let mut num = num;
        for &p in P.iter() {
            if p * p > num {
                break;
            }
            if num % p == 0 {
                prime_indices.entry(p).or_default().push(idx);
                while num % p == 0 {
                    num /= p;
                }
            }
        }
        if num > 1 {
            prime_indices.entry(num).or_default().push(idx);
        }
    }
    let mut queue = VecDeque::from([(0, 0)]);
    let mut seen = vec![false; n];
    seen[0] = true;
    while let Some((node, step)) = queue.pop_front() {
        if node == n - 1 {
            return step;
        }
        if node > 0 && !seen[node - 1] {
            seen[node - 1] = true;
            queue.push_back((node - 1, 1 + step));
        }
        if 1 + node < n && !seen[1 + node] {
            seen[1 + node] = true;
            queue.push_back((1 + node, 1 + step));
        }
        if let Some(v) = prime_indices.remove(&nums[node]) {
            for i in v {
                if !seen[i] {
                    seen[i] = true;
                    queue.push_back((i, 1 + step));
                }
            }
        }
    }
    -1
}

const MAX: usize = 1_000_000;
static P: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut sieve = vec![true; 1 + MAX];
    sieve[..2].fill(false);
    for p in 2..=MAX.isqrt() {
        if sieve[p] {
            for val in (p * p..=MAX).step_by(p) {
                sieve[val] = false
            }
        }
    }
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v { Some(i as i32) } else { None })
        .collect()
});

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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(
            interval_intersection(
                &[[0, 2], [5, 10], [13, 23], [24, 25]],
                &[[1, 5], [8, 12], [15, 24], [25, 26]]
            ),
            [[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
        );
    }

    #[test]
    fn test() {}
}
