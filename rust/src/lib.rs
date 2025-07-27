mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{
    collections::{HashMap, VecDeque},
    sync::LazyLock,
};

#[allow(unused_imports)]
use helper::*;

pub fn min_jumps(nums: &[i32]) -> i32 {
    let mut buckets = HashMap::<_, Vec<_>>::new();
    for (idx, &num) in nums.iter().enumerate() {
        precompute(&mut buckets, idx, num);
    }
    let n = nums.len();
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
        if node < n - 1 && !seen[1 + node] {
            seen[1 + node] = true;
            queue.push_back((1 + node, 1 + step));
        }
        if S[nums[node] as usize]
            && let Some(buc) = buckets.remove(&nums[node])
        {
            for next in buc {
                if !seen[next] {
                    seen[next] = true;
                    queue.push_back((next, 1 + step));
                }
            }
        }
    }
    -1
}

fn precompute(buckets: &mut HashMap<i32, Vec<usize>>, idx: usize, mut num: i32) {
    let mut p = 2;
    while p * p <= num {
        if S[p as usize] && num % p == 0 {
            buckets.entry(p).or_default().push(idx);
            while num % p == 0 {
                num /= p;
            }
        }
        p += 1;
    }
    if num > 1 {
        buckets.entry(num).or_default().push(idx);
    }
}

static S: LazyLock<Vec<bool>> = LazyLock::new(|| {
    let n = 1_000_001;
    let mut sieve = vec![true; n];
    sieve[..2].fill(false);
    for p in 2..=n.isqrt() {
        if sieve[p] {
            for val in (p * p..n).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    sieve
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
    fn basics() {}

    #[test]
    fn test() {
        assert_eq!(min_jumps(&[5, 2, 20, 1, 15]), 1);
    }
}
