mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_repeating(s: String, query_characters: &str, query_indices: &[i32]) -> Vec<i32> {
    let mut s = s.into_bytes();
    let mut spans = BTreeMap::new();
    let mut lengths = BTreeMap::new();
    let mut left = 0;
    for w in s.chunk_by(|a, b| a == b) {
        spans.insert(left, left + w.len() - 1);
        *lengths.entry(w.len()).or_insert(0) += 1;
        left += w.len();
    }
    let mut res = vec![];
    for (qi, qch) in query_indices
        .iter()
        .zip(query_characters.bytes())
        .map(|(&qi, b)| (qi as usize, b))
    {
        if s[qi] != qch {
            let (&left, &right) = spans.range(..=qi).next_back().unwrap();
            spans.remove(&left);
            let length = right + 1 - left;
            *lengths.entry(length).or_insert(0) -= 1;
            if lengths[&length] == 0 {
                lengths.remove(&length);
            }
            if left < qi {
                spans.insert(left, qi - 1);
                *lengths.entry(qi - left).or_insert(0) += 1;
            }
            if qi < right {
                spans.insert(1 + qi, right);
                *lengths.entry(right - qi).or_insert(0) += 1;
            }
            s[qi] = qch;
            let [mut left, mut right] = [qi; 2];
            if qi.checked_sub(1).is_some_and(|i| s[i] == s[qi]) {
                let item = spans.range(..=qi).next_back().unwrap();
                left = *item.0;
                let length = spans.remove(&left).unwrap() + 1 - left;
                *lengths.entry(length).or_insert(0) -= 1;
                if lengths[&length] == 0 {
                    lengths.remove(&length);
                }
            }
            if s.get(1 + qi).is_some_and(|&v| v == s[qi]) {
                let (&key, _) = spans.range(qi..).next().unwrap();
                right = spans.remove(&key).unwrap();
                let length = right + 1 - key;
                *lengths.entry(length).or_insert(0) -= 1;
                if lengths[&length] == 0 {
                    lengths.remove(&length);
                }
            }
            spans.insert(left, right);
            *lengths.entry(right + 1 - left).or_insert(0) += 1;
        }
        res.push(*lengths.keys().last().unwrap() as i32);
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
            longest_repeating("babacc".into(), "bcb", &[1, 3, 3]),
            [3, 3, 4]
        );
        assert_eq!(longest_repeating("abyzz".into(), "aa", &[2, 1]), [2, 3]);
    }

    #[test]
    fn test() {}
}
