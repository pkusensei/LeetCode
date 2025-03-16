mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
    use std::collections::HashMap;
    const MOD: i64 = 1_000_000_007;
    let mut freq: HashMap<_, i64> = (1..=max_value).map(|v| (v, 1)).collect();
    let mut res = 0;
    let mut combi = 1;
    for k in 0..n {
        let mut curr = HashMap::new();
        for (&num, &count) in freq.iter() {
            for val in (num * 2..=max_value).step_by(num as usize) {
                *curr.entry(val).or_insert(0) += count;
            }
        }
        res += combi * freq.values().sum::<i64>() % MOD;
        res %= MOD;
        combi = (combi * i64::from(n - k - 1)) % MOD * mod_pow((1 + k).into(), MOD - 2, MOD) % MOD;
        freq = curr;
    }
    (res % MOD) as i32
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
        assert_eq!(ideal_arrays(2, 5), 10);
        assert_eq!(ideal_arrays(5, 3), 11);
    }

    #[test]
    fn test() {
        assert_eq!(ideal_arrays(5878, 2900), 465040898);
    }
}
