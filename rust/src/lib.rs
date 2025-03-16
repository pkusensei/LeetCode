mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_ideal_string(s: &str, k: i32) -> i32 {
    let s = s.as_bytes();
    let k = k as u8;
    let mut dp = [0; 26];
    for &b in s.iter() {
        let mut curr = dp;
        for i in (b - b'a').saturating_sub(k)..=(b - b'a' + k).min(25) {
            curr[usize::from(b - b'a')] = curr[usize::from(b - b'a')].max(1 + dp[usize::from(i)]);
        }
        dp = curr;
    }
    dp.into_iter().max().unwrap_or(1)
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
        assert_eq!(longest_ideal_string("acfgbd", 2), 4);
        assert_eq!(longest_ideal_string("abcd", 3), 4);
    }

    #[test]
    fn test() {
        assert_eq!(longest_ideal_string("lkpkxcigcs", 6), 7);
    }
}
