mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn zig_zag_arrays_2(n: i32, l: i32, r: i32) -> i32 {
    let width = (r + 1 - l) as usize;
    let mut state = vec![vec![1]; 2 * width];
    let mut transition = vec![vec![0; 2 * width]; 2 * width];
    for i in 0..width {
        for j in 0..i {
            transition[i][j + width] = 1;
        }
        for j in 1 + i..width {
            transition[i + width][j] = 1;
        }
    }
    transition = mat_pow(transition, n - 1);
    state = mat_mul(&transition, &state);
    (0..2 * width).fold(0, |acc, row| (acc + state[row][0]) % M) as i32
}

fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let m = a.len();
    let n = a[0].len();
    let p = b[0].len();
    assert_eq!(
        n,
        b.len(),
        "mat_mul must work on matrices with {m}*{n} and {n}*{p}",
    ); // m*n n*p => m*p
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
    assert_eq!(n, mat[0].len(), "mat_pow must work on square matrix");
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        res[i][i] = 1; // identity matrix
    }
    while pow > 0 {
        if pow & 1 == 1 {
            res = mat_mul(&res, &mat);
        }
        pow >>= 1;
        mat = mat_mul(&mat, &mat);
    }
    res
}

pub fn zig_zag_arrays_1(n: i32, l: i32, r: i32) -> i32 {
    let [n, low, high] = [n, l - l, r - l].map(|v| v as usize);
    let width = high + 1 - low;
    let mut dp_inc = vec![1; width];
    let mut dp_dec = vec![1; width];
    for _ in 1..n {
        let mut curr_inc = vec![0; width];
        let mut curr_dec = vec![0; width];
        for i in 1 + low..=high {
            curr_inc[i] = (dp_dec[i - 1] + curr_inc[i - 1]) % M;
        }
        for i in (low..high).rev() {
            curr_dec[i] = (dp_inc[1 + i] + curr_dec[1 + i]) % M;
        }
        dp_inc = curr_inc;
        dp_dec = curr_dec;
    }
    dp_inc
        .into_iter()
        .chain(dp_dec)
        .fold(0, |acc, v| (acc + v) % M) as i32
    // let [n, left, right] = [n, l, r].map(|v| v as usize);
    // let width = right - left + 1;
    // let mut memo = vec![vec![vec![-1; width]; 2]; n];
    // (left..=right)
    //     .map(|curr| {
    //         (dfs(n, left, right, 0, curr, &mut memo) + dfs(n, left, right, 1, curr, &mut memo)) % M
    //     })
    //     .fold(0, |acc, v| (acc + v) % M)
}

const M: i64 = 1_000_000_007;

// tle's
fn dfs(
    n: usize,
    left: usize,
    right: usize,
    dir: usize,
    curr: usize,
    memo: &mut [Vec<Vec<i64>>],
) -> i64 {
    if n == 1 {
        return 1;
    }
    if memo[n - 1][dir][curr - left] > -1 {
        return memo[n - 1][dir][curr - left];
    }
    let range = if dir == 1 {
        left..curr
    } else {
        1 + curr..1 + right
    };
    let res = range
        .map(|val| dfs(n - 1, left, right, 1 ^ dir, val, memo))
        .fold(0, |acc, v| (acc + v) % M);
    memo[n - 1][dir][curr - left] = res;
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
        assert_eq!(zig_zag_arrays_1(3, 1, 3), 10);
        assert_eq!(zig_zag_arrays_1(3, 4, 5), 2);

        assert_eq!(zig_zag_arrays_2(3, 1, 3), 10);
        assert_eq!(zig_zag_arrays_2(3, 4, 5), 2);
    }

    #[test]
    fn test() {}
}
