mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_substrings(s: &str) -> i64 {
    let mut dp = [[0; 10]; 10];
    let mut res = 0;
    for digit in s.bytes().map(|b| usize::from(b - b'0')) {
        let mut curr = [[0; 10]; 10];
        for den in 1..10 {
            for rem in 0..10 {
                curr[den][(rem * 10 + digit) % den] += dp[den][rem]
            }
            curr[den][digit % den] += 1; // start new substr
        }
        res += curr[digit][0];
        dp = curr;
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
        assert_eq!(count_substrings("1010101010"), 25);
        assert_eq!(count_substrings("12936"), 11);
        assert_eq!(count_substrings("5701283"), 18);
    }

    #[test]
    fn test() {}
}
