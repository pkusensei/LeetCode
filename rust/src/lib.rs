mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn wonderful_substrings(word: &str) -> i64 {
    let mut freq = std::collections::HashMap::from([(0, 1)]);
    let mut res = 0;
    let mut mask = 0;
    for b in word.bytes() {
        mask ^= 1 << (b - b'a');
        res += freq.get(&mask).unwrap_or(&0); // after xor they are all 0
        *freq.entry(mask).or_insert(0) += 1;
        // Try every letter
        for bit in 0..10 {
            res += freq.get(&(mask ^ (1 << bit))).unwrap_or(&0);
        }
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
        assert_eq!(wonderful_substrings("aba"), 4);
        assert_eq!(wonderful_substrings("aabb"), 9);
        assert_eq!(wonderful_substrings("he"), 2);
    }

    #[test]
    fn test() {}
}
