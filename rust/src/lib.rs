mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_least_frequent_digit(mut n: i32) -> i32 {
    let mut freq = [0; 10];
    while n > 0 {
        freq[(n % 10) as usize] += 1;
        n /= 10;
    }
    let mut f = i32::MAX;
    let mut res = 9;
    for (i, v) in freq.into_iter().enumerate() {
        if v > 0 && v < f {
            f = v;
            res = i as i32
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(get_least_frequent_digit(1553322), 1);
        assert_eq!(get_least_frequent_digit(723344511), 2);
    }

    #[test]
    fn test() {}
}
