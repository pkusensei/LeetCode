mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_strings(n: i64) -> i32 {
    let mat = mat_pow([[1, 1], [1, 0]], n);
    (mat[0][1] * 2 % M) as i32
}

type MAT = [[i64; 2]; 2];
const M: i64 = 1_000_000_007;

fn mat_mul(a: MAT, b: MAT) -> MAT {
    let mut res = MAT::default();
    for i1 in 0..2 {
        for i2 in 0..2 {
            for i3 in 0..2 {
                res[i1][i2] = (res[i1][i2] + a[i1][i3] * b[i3][i2]) % M;
            }
        }
    }
    res
}

fn mat_pow(mut mat: MAT, mut pow: i64) -> MAT {
    let mut res = [[1, 0], [0, 1]];
    while pow > 0 {
        if pow & 1 == 1 {
            res = mat_mul(res, mat);
        }
        pow >>= 1;
        mat = mat_mul(mat, mat);
    }
    res
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
