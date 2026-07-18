mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rearrange_string(s: String, x: char, y: char) -> String {
    use std::iter::repeat_n;
    let [x, y] = [x, y].map(|v| v as u8);
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = Vec::with_capacity(s.len());
    res.extend(repeat_n(y, freq[usize::from(y - b'a')]));
    res.extend(repeat_n(x, freq[usize::from(x - b'a')]));
    for (i, &v) in freq.iter().enumerate() {
        let b = i as u8 + b'a';
        if b != x && b != y {
            res.extend(repeat_n(b, v));
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
    fn basics() {}

    #[test]
    fn test() {}
}
