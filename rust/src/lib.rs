mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn string_count(n: i32) -> i32 {
    dfs(n, 0, &mut HashMap::new()) as i32
}

fn dfs(n: i32, mask: i32, memo: &mut HashMap<[i32; 2], i64>) -> i64 {
    const MOD: i64 = 1_000_000_007;
    const LEET: i32 = 0b1111;
    const L: i32 = 0b1000;
    const E1: i32 = 0b0100;
    const E2: i32 = 0b0010;
    const T: i32 = 0b0001;
    if n == 0 {
        return (mask == LEET).into();
    }
    if let Some(&v) = memo.get(&[n, mask]) {
        return v;
    }
    let mut res = (dfs(n - 1, mask | L, memo) + dfs(n - 1, mask | T, memo)) % MOD;
    if mask & E1 > 0 {
        res += dfs(n - 1, mask | E2, memo);
    } else {
        res += dfs(n - 1, mask | E1, memo);
    }
    res %= MOD;
    res += 23 * dfs(n - 1, mask, memo) % MOD;
    res %= MOD;
    memo.insert([n, mask], res);
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
        assert_eq!(string_count(4), 12);
        assert_eq!(string_count(10), 83943898);
    }

    #[test]
    fn test() {}
}
