mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_stack(s: &str) -> i32 {
    let mut res = 0;
    let mut st = vec![];
    for b in s.bytes() {
        if b == b'a' && st.last().is_some_and(|v| *v == b'b') {
            st.pop();
            res += 1;
        } else {
            st.push(b);
        }
    }
    res
}

pub fn with_prefix_suffix(s: &str) -> i32 {
    let mut count_a = 0;
    let mut suf_a = vec![];
    // count of 'a' after this idx
    for b in s.bytes().rev() {
        suf_a.push(count_a);
        count_a += i32::from(b == b'a');
    }
    suf_a.reverse();
    let mut res = s.len() as i32;
    let mut count_b = 0;
    for (b, count_a) in s.bytes().zip(suf_a) {
        res = res.min(count_a + count_b);
        count_b += i32::from(b == b'b');
    }
    res
}

pub fn with_prefix_suffix_opt(s: &str) -> i32 {
    let mut count_a = s.bytes().filter(|&b| b == b'a').count() as i32;
    let mut res = s.len() as i32;
    let mut count_b = 0;
    for b in s.bytes() {
        // 'a' to the right
        count_a -= i32::from(b == b'a');
        res = res.min(count_a + count_b);
        // 'b' to the left
        count_b += i32::from(b == b'b');
    }
    res
}

pub fn with_dp(s: &str) -> i32 {
    let n = s.len();
    let mut dp = vec![0; 1 + n];
    let mut count_b = 0;
    for (i, b) in s.bytes().enumerate() {
        if b == b'b' {
            dp[1 + i] = dp[i];
            count_b += 1;
        } else {
            dp[1 + i] = (1 + dp[i]).min(count_b);
        }
    }
    dp[n]
}

pub fn with_dp_opt(s: &str) -> i32 {
    let mut res = 0;
    let mut count_b = 0;
    for b in s.bytes() {
        if b == b'b' {
            count_b += 1;
        } else {
            res = (1 + res).min(count_b);
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
        assert_eq!(with_stack("aababbab"), 2);
        assert_eq!(with_stack("bbaaaaabb"), 2);

        assert_eq!(with_prefix_suffix("aababbab"), 2);
        assert_eq!(with_prefix_suffix("bbaaaaabb"), 2);

        assert_eq!(with_prefix_suffix_opt("aababbab"), 2);
        assert_eq!(with_prefix_suffix_opt("bbaaaaabb"), 2);

        assert_eq!(with_dp("aababbab"), 2);
        assert_eq!(with_dp("bbaaaaabb"), 2);

        assert_eq!(with_dp_opt("aababbab"), 2);
        assert_eq!(with_dp_opt("bbaaaaabb"), 2);
    }

    #[test]
    fn test() {}
}
