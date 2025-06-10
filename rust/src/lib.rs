mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_matching_substring(s: &str, p: &str) -> i32 {
    use itertools::Itertools;
    let s = s.as_bytes();
    let segs = p
        .split('*')
        .filter_map(|seg| {
            if !seg.is_empty() {
                Some(seg.as_bytes())
            } else {
                None
            }
        })
        .collect_vec();
    let mut res = i32::MAX;
    match segs[..] {
        [] => res = 0,
        [seg] => {
            let arr = kmp(s, seg);
            if !arr.is_empty() {
                res = segs[0].len() as i32;
            }
        }
        [seg1, seg2] => {
            let arr1 = kmp(s, seg1);
            let arr2 = kmp(s, seg2);
            if arr1.is_empty() || arr2.is_empty() {
                return -1;
            }
            for start1 in arr1 {
                let i = arr2.partition_point(|&v| v < start1 + segs[0].len());
                let Some(&start2) = arr2.get(i) else {
                    continue;
                };
                let len = start2 + segs[1].len() - start1;
                res = res.min(len as i32);
            }
        }
        _ => {
            let arr1 = kmp(s, segs[0]);
            let arr2 = kmp(s, segs[1]);
            let arr3 = kmp(s, segs[2]);
            if [&arr1, &arr2, &arr3].iter().any(|a| a.is_empty()) {
                return -1;
            }
            for start1 in arr1 {
                let i = arr2.partition_point(|&v| v < start1 + segs[0].len());
                let Some(&start2) = arr2.get(i) else {
                    continue;
                };
                let i = arr3.partition_point(|&v| v < start2 + segs[1].len());
                let Some(&start3) = arr3.get(i) else {
                    continue;
                };
                let len = start3 + segs[2].len() - start1;
                res = res.min(len as i32);
            }
        }
    }
    if res < i32::MAX { res } else { -1 }
}

fn kmp(hay: &[u8], needle: &[u8]) -> Vec<usize> {
    let n = needle.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    for idx in 1..n {
        while len > 0 && needle[idx] != needle[len] {
            len = lps[len - 1];
        }
        if needle[idx] == needle[len] {
            len += 1;
        }
        lps[idx] = len;
    }
    len = 0;
    let mut matches = vec![];
    for (idx, &val) in hay.iter().enumerate() {
        while len > 0 && (len == n || val != needle[len]) {
            len = lps[len - 1];
        }
        if needle.get(len).is_some_and(|&v| v == val) {
            len += 1;
        }
        if len == n {
            matches.push(idx + 1 - len);
        }
    }
    matches
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
        assert_eq!(shortest_matching_substring("abaacbaecebce", "ba*c*ce"), 8);
        assert_eq!(shortest_matching_substring("baccbaadbc", "cc*baa*adb"), -1);
        assert_eq!(shortest_matching_substring("a", "**"), 0);
        assert_eq!(shortest_matching_substring("madlogic", "*adlogi*"), 6)
    }

    #[test]
    fn test() {
        assert_eq!(shortest_matching_substring("srs", "r**s"), 2);
    }
}
