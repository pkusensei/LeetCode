mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_digitorial_permutation(n: i32) -> bool {
    let mut sum = 0;
    let mut x = n;
    let mut freq = [0; 10];
    while x > 0 {
        let d = x % 10;
        freq[d as usize] += 1;
        x /= 10;
        sum += f(d.into());
    }
    while sum > 0 {
        let d = sum % 10;
        freq[d as usize] -= 1;
        sum /= 10;
    }
    freq.iter().all(|&v| v == 0)
}

const fn f(v: i64) -> i64 {
    if v <= 1 { 1 } else { v * f(v - 1) }
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
    fn basics() {}

    #[test]
    fn test() {}
}
