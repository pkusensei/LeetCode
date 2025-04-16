mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        dfs(n, limit, 3, &mut HashMap::new())
}

fn dfs(n: i32, limit: i32, c: i32, memo: &mut HashMap<[i32; 2], i32>) -> i32 {
    if c == 0 {
        return i32::from(n == 0);
    }
    if n < 0 {
        return 0;
    }
    if let Some(&v) = memo.get(&[n, c]) {
        return v;
    }
    let res = (0..=limit.min(n))
        .map(|v| dfs(n - v, limit, c - 1, memo))
        .sum();
    memo.insert([n, c], res);
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
        assert_eq!(distribute_candies(5, 2), 3);
        assert_eq!(distribute_candies(3, 3), 10);
    }

    #[test]
    fn test() {}
}
