mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_difference(s: String) -> i32 {
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let [mut odd, mut even] = [1, i32::MAX];
    for f in freq {
        if f & 1 == 1 {
            odd = odd.max(f)
        } else if f > 0 {
            even = even.min(f);
        }
    }
    odd - even
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
