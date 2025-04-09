mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
    less_than(high, k) - less_than(low - 1, k)
}

fn less_than(num: i32, k: i32) -> i32 {
    let pos = if num > 0 { 1 + num.ilog10() } else { 1 };
    dfs(num, k, pos, true, true, 0, 0, 0, &mut HashMap::new())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Key(u32, bool, bool, i32, i32, i32);

fn dfs(
    num: i32,
    k: i32,
    pos: u32,
    tight: bool,
    leading: bool,
    odd: i32,
    even: i32,
    rem: i32,
    memo: &mut HashMap<Key, i32>,
) -> i32 {
    if pos == 0 {
        return i32::from(!leading && odd == even && rem == 0);
    }
    let key = Key(pos, tight, leading, odd, even, rem);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let limit = if tight {
        (num / 10i32.pow(pos - 1)) % 10 // THIS!!
    } else {
        9
    };
    let mut res = 0;
    let start = if leading { 1 } else { 0 };
    if leading {
        res += dfs(num, k, pos - 1, false, leading, odd, even, rem, memo);
    }
    for d in start..=limit {
        let next_tight = tight && d == limit;
        res += dfs(
            num,
            k,
            pos - 1,
            next_tight,
            false,
            odd + (d & 1),
            even + (d & 1 ^ 1),
            (10 * rem + d) % k,
            memo,
        );
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
        assert_eq!(less_than(10, 3), 0);
        assert_eq!(number_of_beautiful_integers(10, 20, 3), 2);
        assert_eq!(number_of_beautiful_integers(1, 10, 1), 1);
        assert_eq!(number_of_beautiful_integers(5, 5, 2), 0);
    }

    #[test]
    fn test() {
        assert_eq!(number_of_beautiful_integers(5604, 42522, 19), 90);
        assert_eq!(number_of_beautiful_integers(47, 100, 18), 3);
        assert_eq!(number_of_beautiful_integers(26, 74, 7), 4);
    }
}
