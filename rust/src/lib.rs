mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn length_after_transformations(s: &str, t: i32) -> i32 {
    const M: i32 = 1_000_000_007;
    let mut freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    for _ in 0..t {
        let mut curr = [0; 26];
        curr[1..].copy_from_slice(&freq[..25]);
        curr[0] = freq[25];
        curr[1] = (curr[1] + freq[25]) % M;
        freq = curr;
    }
    freq.into_iter().fold(0, |acc, v| (acc + v) % M)
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
        assert_eq!(length_after_transformations("abcyy", 2), 7);
        assert_eq!(length_after_transformations("azbk", 1), 5);
    }

    #[test]
    fn test() {}
}
