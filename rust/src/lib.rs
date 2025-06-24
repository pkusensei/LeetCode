mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_substrings(word: &str) -> i32 {
    let mut freq = [const { vec![] }; 26];
    for (i, b) in word.bytes().map(|b| usize::from(b - b'a')).enumerate() {
        freq[b].push(i);
    }
    let mut spans = vec![];
    for row in freq {
        if row.len() < 2 {
            continue;
        }
        for &left in &row {
            let i = row.partition_point(|&v| v <= left || v - left < 3);
            if let Some(&right) = row.get(i) {
                spans.push([left, right]);
            } else {
                break;
            }
        }
    }
    spans.sort_unstable_by_key(|&[_, e]| e);
    let mut res = 0;
    let mut last = None;
    for [start, end] in spans {
        if last.is_none_or(|v| v < start) {
            res += 1;
            last = Some(end);
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
        assert_eq!(max_substrings("abcdeafdef"), 2);
        assert_eq!(max_substrings("bcdaaaab"), 1);
    }

    #[test]
    fn test() {
        assert_eq!(max_substrings("abcceaddba"), 1);
    }
}
