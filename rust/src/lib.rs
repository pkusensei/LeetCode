mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn two_egg_drop(n: i32) -> i32 {
    let mut res = 1;
    while res * (res + 1) / 2 < n {
        res += 1;
    }
    res
    // dfs(n, 2, &mut HashMap::new())
}

fn dfs(floors: i32, eggs: i32, memo: &mut HashMap<[i32; 2], i32>) -> i32 {
    if floors <= 1 {
        return floors;
    }
    if eggs <= 1 {
        return floors;
    }
    if let Some(&v) = memo.get(&[floors, eggs]) {
        return v;
    }
    let mut res = i32::MAX;
    for f in 1..=floors {
        // For each floor between 0 and floors
        // The max of
        // (0..f) one egg broken
        // (f..floors) no egg broken
        let curr = dfs(f - 1, eggs - 1, memo).max(dfs(floors - f, eggs, memo));
        res = res.min(curr);
    }
    res += 1;
    memo.insert([floors, eggs], res);
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
        assert_eq!(two_egg_drop(2), 2);
        assert_eq!(two_egg_drop(100), 14);
    }

    #[test]
    fn test() {}
}
