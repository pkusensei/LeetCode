mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_substring_length(s: &str, k: i32) -> bool {
    use itertools::iproduct;
    let mut left = [None; 26];
    let mut right = [None; 26];
    let mut freqs = [0; 26];
    for (idx, bi) in s.bytes().map(|b| usize::from(b - b'a')).enumerate() {
        left[bi].get_or_insert(idx);
        right[bi] = Some(idx);
        freqs[bi] += 1;
    }
    let mut spans = vec![];
    for (i1, i2) in iproduct!(left, right) {
        let Some((i1, i2)) = i1.zip(i2) else {
            continue; // try all [start, end] combos
        };
        if i1 <= i2 {
            let mut count = 0;
            for bi in (0..26).filter(|&b| freqs[b] > 0) {
                if left[bi].is_some_and(|v| i1 <= v) && right[bi].is_some_and(|v| v <= i2) {
                    count += freqs[bi]; // All of this letter is in range
                }
            }
            if count == i2 + 1 - i1 && count < s.len() {
                spans.push([i1, i2]); // Fulfills a substr
            }
        }
    }
    spans.sort_unstable_by_key(|s| s[1] - s[0]);
    let mut res: Vec<[usize; 2]> = vec![];
    for [left, right] in spans {
        if res.iter().all(|&[x, y]| y < left || right < x) {
            res.push([left, right]);
        }
    }
    res.len() >= k as usize
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
        assert!(max_substring_length("abcdbaefab", 2));
        assert!(!max_substring_length("cdefdc", 3));
        assert!(max_substring_length("abeabe", 0));
    }

    #[test]
    fn test() {
        assert!(max_substring_length("cjd", 3));
    }
}
