mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    const M: i32 = 1_000_000_007;

    let width = (1 + r - l) as usize;
    let mut dp_inc = vec![1; width];
    let mut dp_dec = vec![1; width];
    for _ in 1..n {
        let mut curr_inc = vec![0; width];
        let mut curr_dec = vec![0; width];
        for i in 1..width {
            curr_inc[i] = (curr_inc[i - 1] + dp_dec[i - 1]) % M;
        }
        for i in (0..width - 1).rev() {
            curr_dec[i] = (curr_dec[1 + i] + dp_inc[1 + i]) % M;
        }
        dp_inc = curr_inc;
        dp_dec = curr_dec;
    }
    dp_inc
        .into_iter()
        .chain(dp_dec)
        .fold(0, |acc, v| (acc + v) % M)
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
