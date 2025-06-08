mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_increments(nums: &[i32], target: &[i32]) -> i32 {
    let n = target.len();
    let mut mask_lcms = vec![1; 1 << n];
    for mask in 1..1 << n {
        let mut curr = 1;
        for (i, &num) in target.iter().enumerate() {
            if (mask >> i) & 1 == 1 {
                curr = lcm(curr, num.into());
            }
        }
        mask_lcms[mask] = curr;
    }
    let mut dp = vec![i64::MAX; 1 << n];
    dp[0] = 0;
    for &num in nums.iter() {
        let num = i64::from(num);
        let mut mask_costs = vec![0; 1 << n];
        for (mask, &lcm_) in mask_lcms.iter().enumerate().skip(1) {
            let rem = num % lcm_;
            mask_costs[mask] = if rem == 0 { 0 } else { lcm_ - rem };
        }
        let mut curr = dp.clone();
        for prev in 0..1 << n {
            if dp[prev] == i64::MAX {
                continue;
            }
            for (mask, &cost) in mask_costs.iter().enumerate() {
                let new_mask = prev | mask;
                curr[new_mask] = curr[new_mask].min(dp[prev] + cost);
            }
        }
        dp = curr;
    }
    let res = dp[(1 << n) - 1];
    if res < i64::MAX { res as i32 } else { -1 }
}

const fn lcm(a: i64, b: i64) -> i64 {
    const fn gcd(a: i64, b: i64) -> i64 {
        if a == 0 { b } else { gcd(b % a, a) }
    }
    a / gcd(a, b) * b
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
        assert_eq!(minimum_increments(&[1, 2, 3], &[4]), 1);
        assert_eq!(minimum_increments(&[8, 4], &[10, 5]), 2);
        assert_eq!(minimum_increments(&[7, 9, 10], &[7]), 0);
    }

    #[test]
    fn test() {}
}
