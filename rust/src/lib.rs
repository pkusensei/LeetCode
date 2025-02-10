mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_population(logs: &[[i32; 2]]) -> i32 {
    let mut map = std::collections::BTreeMap::new();
    for log in logs.iter() {
        *map.entry(log[0]).or_insert(0) += 1;
        *map.entry(log[1]).or_insert(0) -= 1;
    }
    let mut curr = 0;
    let mut max = 0;
    let mut res = 0;
    for (k, v) in map.iter() {
        curr += v;
        if curr > max {
            max = curr;
            res = *k;
        }
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
    fn basics() {
        assert_eq!(maximum_population(&[[1993, 1999], [2000, 2010]]), 1993);
        assert_eq!(
            maximum_population(&[[1950, 1961], [1960, 1971], [1970, 1981]]),
            1960
        );
    }

    #[test]
    fn test() {}
}
