mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn last_non_empty_string(s: String) -> String {
    use itertools::Itertools;
    let mut freq = [0; 26];
    let mut latest = [0; 26];
    for (i, b) in s.bytes().enumerate() {
        let pos = usize::from(b - b'a');
        freq[pos] += 1;
        latest[pos] = i;
    }
    let max = freq.into_iter().max().unwrap_or(1);
    let res = latest
        .into_iter()
        .enumerate()
        .filter_map(|(b, pos)| {
            if freq[b] == max {
                Some((pos, b as u8 + b'a'))
            } else {
                None
            }
        })
        .sorted_unstable_by_key(|(pos, _b)| *pos)
        .map(|(_, b)| b)
        .collect_vec();
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
    fn basics() {}

    #[test]
    fn test() {}
}
