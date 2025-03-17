mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
    let diff = (start_pos - end_pos).abs();
    if diff > k || (diff - k) & 1 == 1 {
        return 0;
    }
    dfs(k, diff, &mut HashMap::new())
}

fn dfs(k: i32, dist: i32, memo: &mut HashMap<[i32; 2], i32>) -> i32 {
    if k <= dist {
        return i32::from(k == dist);
    }
    if let Some(&v) = memo.get(&[k, dist]) {
        return v - 1;
    }
    let mut res = 1 + dfs(k - 1, 1 + dist, memo) + dfs(k - 1, (dist - 1).abs(), memo);
    res %= 1_000_000_007;
    memo.insert([k, dist], res);
    res - 1
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
        assert_eq!(number_of_ways(1, 2, 3), 3);
        assert_eq!(number_of_ways(2, 5, 10), 0);
    }

    #[test]
    fn test() {}
}
