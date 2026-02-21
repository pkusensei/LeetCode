mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let k = k as usize;
    let mut dp = vec![0; 1 + k];
    let mut moves = 0;
    while dp[k] < n {
        moves += 1;
        for kk in (1..=k).rev() {
            dp[kk] += 1 + dp[kk - 1];
        }
    }
    moves as i32
}

fn dfs(k: i32, n: i32) -> i32 {
    if n < 2 || k == 1 {
        return n;
    }
    let mut res = i32::MAX;
    let mut left = 1;
    let mut right = n;
    while left < right {
        let mid = left + (right - left) / 2;
        let break_ = dfs(k - 1, mid - 1);
        let intact = dfs(k, n - mid);
        let curr = 1 + break_.max(intact);
        res = res.min(curr);
        if break_ < intact {
            left = 1 + mid;
        } else {
            right = mid;
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
        assert_eq!(super_egg_drop(1, 2), 2);
        assert_eq!(super_egg_drop(2, 6), 3);
        assert_eq!(super_egg_drop(3, 14), 4);
    }

    #[test]
    fn test() {
        assert_eq!(super_egg_drop(2, 1), 1);
        assert_eq!(super_egg_drop(3, 25), 5);
    }
}
