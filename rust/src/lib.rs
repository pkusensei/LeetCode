mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut freq = vec![HashMap::new(); n as usize];
    for p in pick.iter() {
        *freq[p[0] as usize].entry(p[1]).or_insert(0) += 1;
    }
    freq.iter()
        .enumerate()
        .filter(|&(i, map)| map.values().any(|&v| v as usize > i))
        .count() as i32
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
