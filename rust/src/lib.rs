mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_deletions(word: &str, k: i32) -> i32 {
    use itertools::Itertools;
    let freq = word
        .bytes()
        .fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
        .into_iter()
        .filter(|&v| v > 0)
        .sorted_unstable()
        .collect_vec();
    let mut res = 0;
    for (i, &v) in freq.iter().enumerate() {
        let mut curr = 0;
        for &f in &freq[i..] {
            curr += (v + k).min(f);
        }
        res = res.max(curr);
    }
    word.len() as i32 - res
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
