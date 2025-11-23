mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_effective(nums: &[i32]) -> i32 {
    const M: i32 = 1_000_000_007;
    let n = nums.len();
    let [mut max, mut total_or] = [0, 0];
    for &num in nums.iter() {
        max = max.max(num);
        total_or |= num;
    }
    if total_or == 0 {
        return 0;
    }
    let width = 1 + max.ilog2();
    let full_mask = 1 << width;
    let mut dp = vec![0; full_mask];
    for &num in nums.iter() {
        dp[num as usize] += 1;
    }
    for bit in 0..width {
        for mask in 0..full_mask {
            if mask & (1 << bit) > 0 {
                let subset = mask ^ (1 << bit);
                // count of numbers in that tick all bits in mask
                // but they may also tick bits not in mask
                dp[mask] += dp[subset];
            }
        }
    }
    let mut pow2 = Vec::with_capacity(n);
    pow2.push(1);
    for _ in 0..n {
        pow2.push(2 * pow2.last().unwrap_or(&1) % M);
    }
    // count of subseqs for this mask
    for v in dp.iter_mut() {
        *v = (pow2[*v as usize] - 1).rem_euclid(M);
    }
    for bit in 0..width {
        for mask in 0..full_mask {
            if mask & (1 << bit) > 0 {
                let subset = mask ^ (1 << bit);
                // By removing dp[subset]
                // dp[mask] = count of subseqs that tick exactly mask
                dp[mask] = (dp[mask] - dp[subset]).rem_euclid(M);
            }
        }
    }
    dp[..total_or as usize]
        .iter()
        .fold(1, |acc, v| (acc + v) % M)
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
        assert_eq!(count_effective(&[1, 2, 3]), 3);
        assert_eq!(count_effective(&[7, 4, 6]), 4);
    }

    #[test]
    fn test() {}
}
