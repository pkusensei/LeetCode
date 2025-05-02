mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_energy(energy: &[i32], k: i32) -> i32 {
    let n = energy.len();
    let k = k as usize;
    let mut dp = vec![0; n];
    let mut res = -100_000;
    for i in 0..n {
        if i + k < n {
            dp[i + k] = dp[i + k].max(dp[i] + energy[i]);
        } else {
            res = res.max(dp[i] + energy[i])
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
        assert_eq!(maximum_energy(&[5, 2, -10, -5, 1], 3), 3);
        assert_eq!(maximum_energy(&[-2, -3, -1], 2), -1);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_energy(&[5, -10, 4, 3, 5, -9, 9, -7], 2), 23);
    }
}
