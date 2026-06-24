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
    let width = (1 + r - l) as usize;

    // [..width] inc half
    // [width..] dec half
    let mut state = vec![vec![1]; 2 * width];
    let mut transition = vec![vec![0; 2 * width]; 2 * width];
    for prev in 0..width {
        for curr in 0..prev {
            // prev > curr
            transition[prev][curr + width] = 1;
        }
        for curr in 1 + prev..width {
            // prev < curr
            transition[prev + width][curr] = 1;
        }
    }
    transition = mat_pow(transition, n - 1);
    state = mat_mul(&transition, &state);
    state.iter().fold(0, |acc, v| (acc + v[0]) % M) as i32
}

const M: i64 = 1_000_000_007;

fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let m = a.len();
    let n = a[0].len();
    let p = b[0].len();
    let mut res = vec![vec![0; p]; m];
    for i1 in 0..m {
        for i2 in 0..p {
            for i3 in 0..n {
                res[i1][i2] += a[i1][i3] * b[i3][i2];
                res[i1][i2] %= M;
            }
        }
    }
    res
}

fn mat_pow(mut mat: Vec<Vec<i64>>, mut pow: i32) -> Vec<Vec<i64>> {
    let n = mat.len();
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        res[i][i] = 1;
    }
    while pow > 0 {
        if pow & 1 == 1 {
            res = mat_mul(&res, &mat);
        }
        mat = mat_mul(&mat, &mat);
        pow >>= 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
