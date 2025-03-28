mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_string(lcp: &[&[i32]]) -> String {
    let n = lcp.len();
    let mut res = vec![0; n];
    let mut curr = b'a' - 1;
    for (r, row) in lcp.iter().enumerate() {
        if res[r] > 0 {
            continue;
        }
        curr += 1;
        if curr > b'z' {
            return String::new();
        }
        for (c, &v) in row.iter().enumerate().skip(r) {
            if v > 0 {
                res[c] = curr
            }
        }
    }
    for r in 0..n {
        for c in 0..n {
            let mut v = *lcp.get(1 + r).and_then(|row| row.get(1 + c)).unwrap_or(&0);
            v = if res[r] == res[c] { 1 + v } else { 0 };
            if lcp[r][c] != v {
                return String::new();
            }
        }
    }
    String::from_utf8(res).unwrap()
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
        assert_eq!(
            find_the_string(&[&[4, 0, 2, 0], &[0, 3, 0, 1], &[2, 0, 2, 0], &[0, 1, 0, 1]]),
            "abab"
        );
        assert_eq!(
            find_the_string(&[&[4, 3, 2, 1], &[3, 3, 2, 1], &[2, 2, 2, 1], &[1, 1, 1, 1]]),
            "aaaa"
        );
        assert_eq!(
            find_the_string(&[&[4, 3, 2, 1], &[3, 3, 2, 1], &[2, 2, 2, 1], &[1, 1, 1, 3]]),
            ""
        );
    }

    #[test]
    fn test() {}
}
