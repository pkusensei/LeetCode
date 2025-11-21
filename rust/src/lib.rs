mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = Vec::with_capacity(s.len());
    for b in order.bytes() {
        res.extend(std::iter::repeat_n(b, freq[usize::from(b - b'a')]));
        freq[usize::from(b - b'a')] = 0;
    }
    for (i, &f) in freq.iter().enumerate() {
        if f > 0 {
            res.extend(std::iter::repeat_n(i as u8 + b'a', f));
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
