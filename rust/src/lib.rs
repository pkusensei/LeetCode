mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let [n, k, mp] = [n, k, max_pts].map(|v| v as usize);
    let mut dp = vec![1.0]; // starts at 0
    let mut window_sum = if k > 0 { 1.0 } else { 0.0 };
    // window [i-mp..=i-1] can reach [i]
    for i in 1..=n {
        // total of mp options
        dp.push(window_sum / mp as f64);
        // [i] can be added to window
        if i < k {
            window_sum += dp[i];
        }
        // [i-mp] was added to window
        if i >= mp && i - mp < k {
            window_sum -= dp[i - mp];
        }
    }
    dp[k..].iter().sum()
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
        float_eq!(new21_game(10, 1, 10), 1.0);
        float_eq!(new21_game(6, 1, 10), 0.6);
        float_eq!(new21_game(21, 17, 10), 0.73278);
    }

    #[test]
    fn test() {}
}
