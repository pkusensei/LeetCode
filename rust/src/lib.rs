mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let mut res = 0;
    let [mut odd, mut even] = [0, 0];
    let mut prefix = 0;
    for num in arr.iter() {
        prefix += num;
        if prefix & 1 == 1 {
            res = (res + 1 + even) % MOD;
            odd = (1 + odd) % MOD;
        } else {
            res = (res + odd) % MOD;
            even = (1 + even) % MOD;
        }
    }
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
    fn basics() {}

    #[test]
    fn test() {}
}
