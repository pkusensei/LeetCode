mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
    use itertools::{Itertools, izip};
    let arr = words
        .windows(2)
        .map(|w| {
            izip!(w[0].bytes(), w[1].bytes())
                .take_while(|(a, b)| a == b)
                .count() as i32
        })
        .collect_vec();
    let mut prefix = vec![0];
    let mut curr = 0;
    for &num in arr.iter() {
        curr = curr.max(num);
        prefix.push(curr);
    }
    let mut suffix = vec![0];
    curr = 0;
    for &num in arr.iter().rev() {
        curr = curr.max(num);
        suffix.push(curr);
    }
    suffix.reverse();
    let n = words.len();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        let mut curr = 0;
        if i > 0 {
            curr = curr.max(prefix[i - 1]);
        }
        if i < n - 1 {
            curr = curr.max(suffix[1 + i])
        }
        if (1..n - 1).contains(&i) {
            curr = curr.max(
                izip!(words[i - 1].bytes(), words[1 + i].bytes())
                    .take_while(|(a, b)| a == b)
                    .count() as i32,
            )
        }
        res.push(curr);
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
    fn basics() {}

    #[test]
    fn test() {}
}
