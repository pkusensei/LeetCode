mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn distinct_sequences(n: i32) -> i32 {
    dfs(n, 0, 0, &mut HashMap::new()) as _
}

fn dfs(n: i32, x1: i32, x2: i32, memo: &mut HashMap<[i32; 3], i64>) -> i64 {
    if n == 0 {
        return 1;
    }
    let key = [n, x1, x2];
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let mut res = 0;
    match x2 {
        1 => {
            for v in 2..=6 {
                if v != x1 {
                    res += dfs(n - 1, x2, v, memo)
                }
            }
        }
        2 => {
            for v in [1, 3, 5] {
                if v != x1 {
                    res += dfs(n - 1, x2, v, memo)
                }
            }
        }
        3 => {
            for v in [1, 2, 4, 5] {
                if v != x1 {
                    res += dfs(n - 1, x2, v, memo)
                }
            }
        }
        4 => {
            for v in [1, 3, 5] {
                if v != x1 {
                    res += dfs(n - 1, x2, v, memo)
                }
            }
        }
        5 => {
            for v in [1, 2, 3, 4, 6] {
                if v != x1 {
                    res += dfs(n - 1, x2, v, memo)
                }
            }
        }
        6 => {
            for v in [1, 5] {
                if v != x1 {
                    res += dfs(n - 1, x2, v, memo)
                }
            }
        }
        _ => {
            for v in 1..=6 {
                if v != x1 {
                    res += dfs(n - 1, x2, v, memo)
                }
            }
        }
    }
    res %= 1_000_000_007;
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
        assert_eq!(distinct_sequences(4), 184);
        assert_eq!(distinct_sequences(2), 22);
    }

    #[test]
    fn test() {}
}
