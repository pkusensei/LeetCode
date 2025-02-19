mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_unique_good_subsequences(binary: &str) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let [mut end0, mut end1] = [0, 0];
    let mut zero = 0; // "0"
    for b in binary.bytes() {
        if b == b'0' {
            end0 = (end0 + end1) % MOD;
            zero = 1;
        } else {
            end1 = (1 + end0 + end1) % MOD;
        }
    }
    (end0 + end1 + zero) % MOD
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
        assert_eq!(number_of_unique_good_subsequences("001"), 2);
        assert_eq!(number_of_unique_good_subsequences("11"), 2);
        assert_eq!(number_of_unique_good_subsequences("101"), 5);
    }

    #[test]
    fn test() {}
}
