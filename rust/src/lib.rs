mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;
use itertools::{Itertools, izip};

pub fn can_make_palindrome_queries(s: &str, queries: &[[i32; 4]]) -> Vec<bool> {
    let n = s.len();
    let mut freq = [0; 26];
    let func = move |mut acc: Vec<_>, b| {
        freq[usize::from(b - b'a')] += 1;
        acc.push(freq);
        acc
    };
    let left_freq = s[..n / 2].bytes().fold(Vec::with_capacity(n / 2), func);
    let right_freq = s[n / 2..]
        .bytes()
        .rev()
        .fold(Vec::with_capacity(n / 2), func);
    let equals = izip!(s.bytes(), s.bytes().rev()).take(n / 2).fold(
        Vec::with_capacity(n / 2),
        |mut acc, (a, b)| {
            acc.push(i32::from(a == b) + acc.last().unwrap_or(&0));
            acc
        },
    );
    let right_half = s[n / 2..].bytes().rev().collect_vec();
    let mut res = vec![];
    let mut memo = HashMap::new();
    'outer: for q in queries.iter() {
        let [a, b, _c, _d] = [0, 1, 2, 3].map(|i| q[i] as usize);
        let k = [a, b, _c, _d];
        if let Some(&v) = memo.get(&k) {
            res.push(v);
            continue;
        }
        let c = n - 1 - _d;
        let d = n - 1 - _c;
        let min = a.min(c);
        let max = b.max(d);
        // left_equal
        if min > 0 && equals[min - 1] != min as i32 {
            res.push(false);
            memo.insert(k, false);
            continue;
        };
        let right_equal = equals.last().unwrap() - equals[max];
        if right_equal != (n / 2 - max - 1) as i32 {
            res.push(false);
            memo.insert(k, false);
            continue;
        }
        let mut range1 = range(&left_freq, a, b);
        let mut range2 = range(&right_freq, c, d);
        for idx in min..=max {
            match [(a..=b).contains(&idx), (c..=d).contains(&idx)] {
                [false, false] if s.as_bytes()[idx] != right_half[idx] => {
                    res.push(false);
                    memo.insert(k, false);
                    continue 'outer;
                }
                [true, false] => {
                    range1[usize::from(right_half[idx] - b'a')] -= 1;
                    if range1[usize::from(right_half[idx] - b'a')] < 0 {
                        res.push(false);
                        memo.insert(k, false);
                        continue 'outer;
                    }
                }
                [false, true] => {
                    range2[usize::from(s.as_bytes()[idx] - b'a')] -= 1;
                    if range2[usize::from(s.as_bytes()[idx] - b'a')] < 0 {
                        res.push(false);
                        memo.insert(k, false);
                        continue 'outer;
                    }
                }
                _ => (),
            }
        }
        memo.insert(k, range1 == range2);
        res.push(range1 == range2);
    }
    res
}

fn range(prefix: &[[i32; 26]], left: usize, right: usize) -> [i32; 26] {
    let mut res = prefix[right];
    if left > 0 {
        for (v, d) in res.iter_mut().zip(prefix[left - 1]) {
            *v -= d;
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
            can_make_palindrome_queries("aaaaaa", &[[0, 0, 5, 5]]),
            [true]
        );
        assert_eq!(
            can_make_palindrome_queries("abcabc", &[[1, 1, 3, 5], [0, 2, 5, 5]]),
            [true, true]
        );
        assert_eq!(
            can_make_palindrome_queries("abbcdecbba", &[[0, 2, 7, 9]]),
            [false]
        );
        assert_eq!(
            can_make_palindrome_queries("acbcab", &[[1, 2, 4, 5]]),
            [true]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            can_make_palindrome_queries("odaxusaweuasuoeudxwa", &[[0, 5, 10, 14]]),
            [false]
        )
    }
}
