mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(nums: &[i32]) -> i32 {
        let n = nums.len();
        let mut gcdm = HashMap::new();
        backtrack(nums, 0, &mut gcdm, &mut vec![-1; 1 << n])
}

fn backtrack(
    nums: &[i32],
    mask: usize,
    gcdm: &mut HashMap<[i32; 2], i32>,
    memo: &mut [i32],
) -> i32 {
    let n = nums.len();
    if mask.count_ones() as usize == n {
        return 0;
    }
    if memo[mask] > -1 {
        return memo[mask];
    }
    let mut res = 0;
    let coeff = (n as i32 - mask.count_ones() as i32) / 2;
    for i1 in 0..n {
        for i2 in 1 + i1..n {
            if (mask >> i1) & 1 == 1 || (mask >> i2) & 1 == 1 {
                continue;
            }
            let [a, b] = [i1, i2].map(|v| nums[v]);
            res = res.max(
                coeff * gcd(a, b, gcdm) + backtrack(nums, mask | (1 << i1) | (1 << i2), gcdm, memo),
            )
        }
    }
    memo[mask] = res;
    res
}

fn gcd(a: i32, b: i32, memo: &mut HashMap<[i32; 2], i32>) -> i32 {
    if let Some(&v) = memo.get(&[a, b]) {
        return v;
    }
    let res = if a == 0 { b } else { gcd(b % a, a, memo) };
    memo.insert([a, b], res);
    res
}

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
        assert_eq!(max_score(&[1, 2]), 1);
        assert_eq!(max_score(&[3, 4, 6, 8]), 11);
        assert_eq!(max_score(&[1, 2, 3, 4, 5, 6]), 14);
    }

    #[test]
    fn test() {}
}
