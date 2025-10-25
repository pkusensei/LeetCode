mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_coprime(mat: Vec<Vec<i32>>) -> i32 {
    let rows = mat.len();
    let mut dp = vec![[None; 151]; rows];
    let mut res = 0;
    for &v in mat[0].iter() {
        res = (res + dfs(&mat, 0, v, &mut dp)) % M;
    }
    res as i32
}

fn dfs(mat: &[Vec<i32>], row: usize, val: i32, dp: &mut [[Option<usize>; 151]]) -> usize {
    let [rows, cols] = get_dimensions(mat);
    if row == rows - 1 {
        return (val == 1).into();
    }
    if let Some(v) = dp[row][val as usize] {
        return v;
    }
    let res = if val == 1 {
        (0..rows - row - 1).fold(1, |acc, _| acc * cols % M)
    } else {
        let mut res = 0;
        for &v in mat[1 + row].iter() {
            res = (res + dfs(mat, 1 + row, gcd(val, v), dp)) % M;
        }
        res
    };
    dp[row][val as usize] = Some(res);
    res
}

const M: usize = 1_000_000_007;

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
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
        assert_eq!(count_coprime(vec![vec![1, 2], vec![3, 4]]), 3);
        assert_eq!(count_coprime(vec![vec![2, 2], vec![2, 2]]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(count_coprime(vec![vec![1]]), 1);
    }
}
