mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_reach_stair(k: i32) -> i32 {
    dfs(k, 1, 0, true, &mut HashMap::new())
}

fn dfs(k: i32, curr: i32, jump: u32, dec: bool, memo: &mut HashMap<(i32, u32, bool), i32>) -> i32 {
    if curr > 1 + k {
        return 0;
    }
    let key = (curr, jump, dec);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let mut res = i32::from(k == curr);
    res += dfs(k, curr + 2_i32.pow(jump), 1 + jump, true, memo);
    if dec && curr > 0 {
        res += dfs(k, curr - 1, jump, false, memo);
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
        assert_eq!(ways_to_reach_stair(0), 2);
        assert_eq!(ways_to_reach_stair(1), 4);
    }

    #[test]
    fn test() {}
}
