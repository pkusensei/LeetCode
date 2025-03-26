mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_win(prize_positions: &[i32], k: i32) -> i32 {
    let n = prize_positions.len();
    let mut dp = vec![0; 1 + n]; // prev largest window
    let mut res = 0;
    let mut left = 0;
    for (right, &num) in prize_positions.iter().enumerate() {
        while num - prize_positions[left] > k {
            left += 1;
        }
        // record max window until right
        dp[1 + right] = dp[right].max(right + 1 - left);
        res = res.max(right + 1 - left + dp[left]);
    }
    res as _
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
        assert_eq!(maximize_win(&[1, 1, 2, 2, 3, 3, 5], 2), 7);
        assert_eq!(maximize_win(&[1, 2, 3, 4], 0), 2);
    }

    #[test]
    fn test() {}
}
