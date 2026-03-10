mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    const M: i32 = 1_000_000_007;
    let [num_zero, num_one, limit] = [zero, one, limit].map(|v| v as usize);
    let mut dp0 = vec![vec![0; 1 + num_one]; 1 + num_zero];
    let mut dp1 = vec![vec![0; 1 + num_one]; 1 + num_zero];
    for i in 1..=num_zero.min(limit) {
        dp0[i][0] = 1;
    }
    for i in 1..=num_one.min(limit) {
        dp1[0][i] = 1;
    }
    for zero in 1..=num_zero {
        for one in 1..=num_one {
            dp0[zero][one] = (dp0[zero - 1][one] + dp1[zero - 1][one]) % M;
            dp1[zero][one] = (dp0[zero][one - 1] + dp1[zero][one - 1]) % M;
            if zero > limit {
                dp0[zero][one] = (dp0[zero][one] - dp1[zero - 1 - limit][one]).rem_euclid(M);
            }
            if one > limit {
                dp1[zero][one] = (dp1[zero][one] - dp0[zero][one - 1 - limit]).rem_euclid(M);
            }
        }
    }
    (dp0[num_zero][num_one] + dp1[num_zero][num_one]) % M
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
