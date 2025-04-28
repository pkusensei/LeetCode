mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_levels(mut possible: Vec<i32>) -> i32 {
    let mut sum = 0;
    for v in possible.iter_mut() {
        if *v == 0 {
            *v = -1;
        }
        sum += *v;
    }
    let mut prefix = 0;
    for (i, &v) in possible.iter().enumerate() {
        prefix += v;
        sum -= v;
        if prefix > sum && i < possible.len() - 1 {
            return 1 + i as i32;
        }
    }
    -1
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
    fn test() {
        assert_eq!(minimum_levels(vec![0, 1, 0]), 2);
    }
}
