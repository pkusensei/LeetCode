mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn winner_square_game(n: i32) -> bool {
    let n = n as usize;
    let mut dp = vec![false; 1 + n];
    for right in 1..=n {
        let mut left = 1_usize;
        while left.pow(2) <= right {
            if !dp[right - left.pow(2)] {
                dp[right] = true;
                break;
            }
            left += 1
        }
    }
    dp[n]
    // dfs(n)
}

fn dfs(n: i32) -> bool {
    if is_square(n) {
        return true;
    }
    for i in 1..=n.isqrt() {
        if !dfs(n - i.pow(2)) {
            return true;
        }
    }
    false
}

const fn is_square(n: i32) -> bool {
    let r = n.isqrt();
    r.pow(2) == n
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
