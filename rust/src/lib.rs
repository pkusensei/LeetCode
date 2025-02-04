mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_homogenous(s: &str) -> i32 {
    s.as_bytes()
        .chunk_by(|a, b| a == b)
        .map(|w| {
            let n = w.len() as i64;
            (n + 1) * n / 2 % 1_000_000_007
        })
        .fold(0, |acc, num| (acc + num) % 1_000_000_007) as _
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(count_homogenous("abbcccaa"), 13);
        assert_eq!(count_homogenous("xy"), 2);
        assert_eq!(count_homogenous("zzzzz"), 15);
    }

    #[test]
    fn test() {}
}
