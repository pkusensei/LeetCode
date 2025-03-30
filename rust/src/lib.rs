mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn partition_labels(s: &str) -> Vec<i32> {
    let n = s.len();
    // last occurrence of byte
    let latest = s
        .bytes()
        .enumerate()
        .rev()
        .fold([n; 26], |mut acc, (i, b)| {
            let pos = usize::from(b - b'a');
            if acc[pos] == n {
                acc[pos] = i;
            }
            acc
        });
    let mut end = 0;
    let mut prev = -1;
    let mut res = vec![];
    for (idx, b) in s.bytes().enumerate() {
        let last = latest[usize::from(b - b'a')];
        end = end.max(last);
        if end == idx {
            res.push(end as i32 - prev);
            prev = end as i32;
            end = 1 + idx;
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
        assert_eq!(partition_labels("ababcbacadefegdehijhklij"), [9, 7, 8]);
        assert_eq!(partition_labels("eccbbbbdec"), [10]);
    }

    #[test]
    fn test() {}
}
