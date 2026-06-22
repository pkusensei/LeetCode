mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_number_of_balloons(text: String) -> i32 {
    let freq = text.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = None;
    for &b in b"ban" {
        let v = res.get_or_insert(freq[usize::from(b - b'a')]);
        *v = (*v).min(freq[usize::from(b - b'a')]);
    }
    for &b in b"lo" {
        let v = res.get_or_insert(freq[usize::from(b - b'a')] / 2);
        *v = (*v).min(freq[usize::from(b - b'a')] / 2);
    }
    res.unwrap_or(0)
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
