mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut curr = 1;
    let mut seen = HashSet::new();
    let mut x = 1;
    while seen.insert(curr) {
        curr = (curr + x * k) % n;
        x += 1;
    }
    (0..n)
        .filter(|v| !seen.contains(v))
        .map(|v| if v == 0 { n } else { v })
        .sorted_unstable()
        .collect()
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
    fn basics() {}

    #[test]
    fn test() {}
}
