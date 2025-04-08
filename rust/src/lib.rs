mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_stepping_numbers(low: String, high: &str) -> i32 {
    let mut low = low.into_bytes();
    for v in low.iter_mut().rev() {
        if *v == b'0' {
            *v = b'9'
        } else {
            *v -= 1;
            break;
        }
    }
    (count_less_than(high.as_bytes()) - count_less_than(&low)).rem_euclid(MOD)
}

const MOD: i32 = 1_000_000_007;

fn count_less_than(s: &[u8]) -> i32 {
    // -1 => the seq of all (-1)'s
    dfs(s, 0, true, -1, &mut HashMap::new()) - 1
}

fn dfs(
    s: &[u8],
    idx: usize,
    tight: bool,
    last: i32,
    memo: &mut HashMap<(usize, bool, i32), i32>,
) -> i32 {
    if idx == s.len() {
        return 1;
    }
    let key = (idx, tight, last);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let max_d = if tight { i32::from(s[idx] - b'0') } else { 9 };
    let mut res = 0;
    for d in 0..=max_d {
        let next_tight = tight && d == max_d;
        if last == -1 {
            let digit = if d == 0 { -1 } else { d };
            res += dfs(s, 1 + idx, next_tight, digit, memo);
        } else if last.abs_diff(d) == 1 {
            res += dfs(s, 1 + idx, next_tight, d, memo);
        }
        res %= MOD;
    }
    memo.insert(key, res);
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
        assert_eq!(count_less_than(b"11"), 10);
        assert_eq!(count_less_than(b"0"), 0);
        assert_eq!(count_stepping_numbers("1".into(), "11"), 10);
        assert_eq!(count_stepping_numbers("90".into(), "101"), 2);
    }

    #[test]
    fn test() {
        assert_eq!(count_less_than(b"149"), 29);
        assert_eq!(count_less_than(b"17"), 11);
        assert_eq!(count_stepping_numbers("17".into(), "149"), 18);
    }
}
