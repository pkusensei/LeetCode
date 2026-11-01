mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// ... value ...
//       ^
//     index
pub fn minimum_max_value(n: i32, index: i32, value: i32) -> i64 {
    let [n, index, value] = [n, index, value].map(i64::from);
    let mut factor = -1;
    for p in 2..=value.isqrt() {
        if value % p == 0 {
            factor = p;
            break;
        }
    }
    if factor == -1 && index == 0 {
        factor = value;
    }
    let left = prev(value, factor);
    if factor < 2 || left < 1 + index {
        return -1;
    }
    if index == n - 1 {
        return value;
    }
    let mut n = n - index - 1;
    let mut res = value;
    while res & 1 == 1 && n > 0 {
        res += factor;
        n -= 1;
    }
    res + 2 * n.max(0)
}

const fn prev(mut value: i64, f: i64) -> i64 {
    let mut res = 0;
    while value & 1 == 1 {
        value -= f;
        res += 1;
    }
    res + value / 2
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn basics() {}

    #[test]
    fn test() {}
}
