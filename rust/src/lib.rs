mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut res = 0;
    for &num in &hours {
        let rem = num % 24;
        if rem == 0 {
            res += map.get(&0).unwrap_or(&0);
        } else {
            res += map.get(&(24 - rem)).unwrap_or(&0);
        }
        *map.entry(num % 24).or_insert(0) += 1;
    }
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
    fn basics() {}

    #[test]
    fn test() {}
}
