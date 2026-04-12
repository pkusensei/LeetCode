mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_distance(word: String) -> i32 {
    let n = word.len();
    let s = word.as_bytes();
    // dp[finger1][finger2][typed_count]
    let mut dp = vec![vec![vec![i32::MAX >> 1; n]; 26]; 26];
    for i in 0..26 {
        let first = usize::from(s[0] - b'A');
        dp[first][i][0] = 0;
        dp[i][first][0] = 0;
    }
    for i in 1..n {
        let curr = usize::from(s[i] - b'A');
        for f1 in 0..26 {
            for f2 in 0..26 {
                dp[curr][f2][i] = dp[curr][f2][i].min(dist(f1, curr) + dp[f1][f2][i - 1]);
                dp[f1][curr][i] = dp[f1][curr][i].min(dist(f2, curr) + dp[f1][f2][i - 1]);
            }
        }
    }
    let mut res = i32::MAX;
    for f1 in 0..26 {
        for f2 in 0..26 {
            res = res.min(dp[f1][f2][n - 1])
        }
    }
    res
}

const fn dist(a: usize, b: usize) -> i32 {
    let [ar, ac] = pos(a);
    let [br, bc] = pos(b);
    (ar.abs_diff(br) + ac.abs_diff(bc)) as i32
}

// letter -> [row, col]
const fn pos(val: usize) -> [usize; 2] {
    [val / 6, val % 6]
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
