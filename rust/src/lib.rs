mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_xor(s: String, t: String) -> String {
    let mut freq = t.bytes().fold([0; 2], |mut acc, b| {
        acc[usize::from(b - b'0')] += 1;
        acc
    });
    let mut res = vec![];
    for b in s.bytes() {
        let bit = usize::from(b - b'0');
        let xor = 1 ^ bit;
        if freq[xor] > 0 {
            freq[xor] -= 1;
            res.push(1 + b'0');
        } else {
            freq[bit] -= 1;
            res.push(0 + b'0');
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
