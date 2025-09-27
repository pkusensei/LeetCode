mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn majority_frequency_group(s: &str) -> String {
    use itertools::Itertools;
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let map = freq.into_iter().filter(|&v| v > 0).counts();
    let max = *map.values().max().unwrap_or(&0);
    let mut f = 0;
    for (k, v) in map {
        if v == max {
            f = f.max(k);
        }
    }
    let mut res = vec![];
    for (i, &v) in freq.iter().enumerate() {
        if v == f {
            res.push(i as u8 + b'a');
        }
    }
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
        assert_eq!(majority_frequency_group("aaabbbccdddde"), "ab");
        assert_eq!(majority_frequency_group("abcd"), "abcd");
        assert_eq!(majority_frequency_group("pfpfgi"), "fp");
    }

    #[test]
    fn test() {}
}
