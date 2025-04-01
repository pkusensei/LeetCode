mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    nums.sort_unstable();
    let mut prefix = 0;
    let mut res = 0;
    for &num in nums.iter() {
        let num = i64::from(num);
        res += (prefix + num) * num % MOD * num % MOD;
        res %= MOD;
        // prefix * 2
        // [1, 2, 3], when accumulating on [3], there are
        // [1, 3]
        // [2, 3]
        // [1, (2), 3] where 2 is omitted
        // Generally, for [.. a, b ..]
        // [a] contributes twice as much as b does
        prefix = (prefix * 2 % MOD + num) % MOD;
    }
    res as i32
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
        assert_eq!(sum_of_power(vec![2, 1, 4]), 141);
        assert_eq!(sum_of_power(vec![1, 1, 1]), 7);
    }

    #[test]
    fn test() {}
}
