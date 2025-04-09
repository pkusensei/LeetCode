mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_sum(n: i32, k: i32) -> i32 {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut num = 1;
    while set.len() < n as usize {
        while num < k && set.contains(&(k - num)) {
            num += 1;
        }
        set.insert(num);
        num += 1;
    }
    set.into_iter().sum()
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
