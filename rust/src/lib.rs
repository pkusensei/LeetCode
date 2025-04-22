mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_indices(s: &str, a: &str, b: &str, k: i32) -> Vec<i32> {
    let lps_a = kmp(a.as_bytes());
    let lps_b = kmp(b.as_bytes());
    let match_a = search(s.as_bytes(), a.as_bytes(), &lps_a);
    let match_b = search(s.as_bytes(), b.as_bytes(), &lps_b);
    let mut res = vec![];
    let mut ia = 0;
    let mut ib = 0;
    while let Some(&va) = match_a.get(ia) {
        while let Some(&vb) = match_b.get(ib) {
            if va.abs_diff(vb) <= k as usize {
                res.push(va as i32);
                break;
            } else if match_b
                .get(1 + ib)
                .is_some_and(|&next_val| next_val.abs_diff(va) < vb.abs_diff(va))
            {
                ib += 1;
            } else {
                break;
            }
        }
        ia += 1;
    }
    res
}

fn kmp(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut lps = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        while j > 0 && s[i] != s[j] {
            j = lps[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        lps[i] = j;
    }
    lps
}

fn search(s: &[u8], pat: &[u8], lps: &[usize]) -> Vec<usize> {
    let mut res = vec![];
    let mut j = 0;
    for (i, &b) in s.iter().enumerate() {
        while j > 0 && b != pat[j] {
            j = lps[j - 1];
        }
        if b == pat[j] {
            j += 1;
        }
        if j == pat.len() {
            res.push(1 + i - j);
            j = lps[j - 1];
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
            beautiful_indices("isawsquirrelnearmysquirrelhouseohmy", "my", "squirrel", 15),
            [16, 33]
        );
        assert_eq!(beautiful_indices("abcd", "a", "a", 4), [0]);
    }

    #[test]
    fn test() {}
}
