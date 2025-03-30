mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let map: HashMap<_, _> = chars.bytes().zip(vals).collect();
    let mut res = 0;
    let mut curr = 0;
    for b in s.bytes() {
        let score = map.get(&b).copied().unwrap_or(i32::from(b - b'a' + 1));
        curr = score.max(curr + score);
        res = res.max(curr)
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
