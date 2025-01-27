mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit(inventory: &mut [i32], orders: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    inventory.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    let n = inventory.len();
    let mut orders = i64::from(orders);
    let mut idx = 0; // width
    let mut curr = i64::from(inventory[0]);
    let mut res = 0;
    while orders > 0 {
        while idx < n && i64::from(inventory[idx]) == curr {
            idx += 1;
        }
        let next = if idx == n {
            0
        } else {
            i64::from(inventory[idx])
        };
        let mut diff = curr - next;
        let mut rem = 0;
        let count = orders.min(idx as i64 * diff);
        if orders < diff * idx as i64 {
            diff = orders / idx as i64;
            rem = orders % idx as i64;
        }
        let val = curr - diff;
        res += (curr + val + 1) * diff / 2 * idx as i64 + val * rem;
        res %= MOD;
        orders -= count;
        curr = next;
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(max_profit(&mut [2, 5], 4), 14);
        assert_eq!(max_profit(&mut [3, 5], 6), 19);
    }

    #[test]
    fn test() {
        assert_eq!(
            max_profit(&mut [497978859, 167261111, 483575207, 591815159], 836556809),
            373219333
        );
        assert_eq!(max_profit(&mut [1000000000], 1000000000), 21);
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
