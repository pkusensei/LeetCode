mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn repeated_string_match(a: &str, b: &str, x: i32, y: i32) -> bool {
    let [a, b] = [&a, &b].map(|s| s.as_bytes());
    let [na, nb] = [a, b].map(|s| s.len());
    let [x, y] = [x, y].map(|v| v as usize);
    let tot_a = na * x;
    let tot_b = nb * y;
    if tot_a < tot_b {
        return false;
    }
    let len = tot_b.min(na + nb);
    for start in 0..na {
        if start + tot_b > tot_a {
            return false;
        }
        let possible = (0..len).all(|i| b[i % nb] == a[(start + i) % na]);
        if possible {
            return true;
        }
    }
    false
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
