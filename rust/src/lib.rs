mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::chain;
use std::iter::{once, repeat_n};

pub fn largest_palindrome(n: i32, k: i32) -> String {
    let n = n as usize;
    match k {
        1 | 3 | 9 => repeat_n('9', n).collect(),
        5 => build('5', n, 2),
        2 => build('8', n, 2),
        4 => build('8', n, 4),
        8 => build('8', n, 6),
        6 => {
            if n <= 2 {
                repeat_n('6', n).collect()
            } else if n & 1 == 1 {
                let block = n / 2 - 1;
                chain!(
                    once('8'),
                    repeat_n('9', block),
                    once('8'),
                    repeat_n('9', block),
                    once('8')
                )
                .collect()
            } else {
                let block = n / 2 - 2;
                chain!(
                    once('8'),
                    repeat_n('9', block),
                    repeat_n('7', 2),
                    repeat_n('9', block),
                    once('8')
                )
                .collect()
            }
        }
        _ => {
            // dreaded 7
            if n <= 2 {
                repeat_n('7', n).collect()
            } else {
                const M: [&str; 12] =
                    ["5", "77", "7", "", "4", "44", "6", "44", "4", "", "7", "77"];
                let mid = M[(n + 9) % 12]; // (n-3)%12
                let len = n.saturating_sub(mid.len()) / 2;
                let mut res = String::with_capacity(n);
                res.extend(repeat_n('9', len));
                res.push_str(mid);
                res.extend(repeat_n('9', len));
                res
            }
        }
    }
}

fn build(d: char, n: usize, min_len: usize) -> String {
    if n <= min_len {
        repeat_n(d, n).collect()
    } else {
        chain!(
            repeat_n(d, min_len / 2),
            repeat_n('9', n - min_len),
            repeat_n(d, min_len / 2)
        )
        .collect()
    }
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
        assert_eq!(largest_palindrome(5, 6), "89898");
    }

    #[test]
    fn test() {}
}
