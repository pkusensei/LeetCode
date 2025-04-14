mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_sub_multisets(nums: &[i32], l: i32, r: i32) -> i32 {
    use std::collections::HashMap;
    const MOD: i64 = 1_000_000_007;
    let freq: HashMap<i32, i64> = nums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let mut dp = vec![0_i64; 1 + r as usize];
    dp[0] = 1;
    for (&num, &count) in freq.iter() {
        for right in ((r - num).max(0) + 1..=r).rev() {
            let mut sum = 0;
            for k in 0..count {
                let i = right - num * k as i32;
                if i < 0 {
                    break;
                }
                sum += dp[i as usize];
                sum %= MOD;
            }
            for left in (1..=right).rev().step_by(num as usize) {
                if left - count as i32 * num >= 0 {
                    sum += dp[(left - count as i32 * num) as usize];
                    sum %= MOD;
                }
                sum -= dp[left as usize];
                sum = sum.rem_euclid(MOD);
                dp[left as usize] += sum;
                dp[left as usize] %= MOD;
            }
        }
    }
    let res = dp[l as usize..=r as usize]
        .iter()
        .fold(0, |acc, &v| (acc + v) % MOD);
    (res * (1 + *freq.get(&0).unwrap_or(&0)) % MOD) as i32
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
        assert_eq!(count_sub_multisets(&[1, 2, 2, 3], 6, 6), 1);
        assert_eq!(count_sub_multisets(&[2, 1, 4, 2, 7], 1, 5), 7);
        assert_eq!(count_sub_multisets(&[1, 2, 1, 3, 5, 2], 3, 5), 9);
    }

    #[test]
    fn test() {}
}
