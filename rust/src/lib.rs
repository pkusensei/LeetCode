mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn length_after_transformations(s: &str, t: i32, nums: &[i32]) -> i32 {
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut mat = Mat::default();
    for (idx, &num) in nums.iter().enumerate() {
        for val in 1..=num {
            mat[(idx + val as usize) % N][idx] += 1; // mat[to][from]
        }
    }
    mat = mat_pow(&mat, t);
    let mut res = [0; N];
    for i in 0..N {
        for j in 0..N {
            res[i] += mat[i][j] * freq[j] % M;
            res[i] %= M;
        }
    }
    res.iter().fold(0, |acc, v| (acc + v) % M) as i32
}

const N: usize = 26;
const M: i64 = 1_000_000_007;
type Mat = [[i64; N]; N];

fn mat_mul(a: &Mat, b: &Mat) -> Mat {
    let mut res = Mat::default();
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                res[i][j] += a[i][k] * b[k][j] % M;
                res[i][j] %= M;
            }
        }
    }
    res
}

fn mat_pow(mat: &Mat, power: i32) -> Mat {
    let mut res = Mat::default();
    for i in 0..N {
        res[i][i] = 1; // identity matrix
    }
    if power == 0 {
        return res;
    }
    if power == 1 {
        return *mat;
    }
    let half = mat_pow(mat, power >> 1);
    let squared = mat_mul(&half, &half);
    if power & 1 == 0 {
        squared
    } else {
        mat_mul(&squared, mat)
    }
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
        assert_eq!(
            length_after_transformations(
                "abcyy",
                2,
                &[
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2
                ]
            ),
            7
        );
        assert_eq!(
            length_after_transformations(
                "azbk",
                1,
                &[
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
                ]
            ),
            8
        );
    }

    #[test]
    fn test() {}
}
