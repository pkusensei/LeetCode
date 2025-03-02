mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_ways(corridor: &str) -> i32 {
    let count = corridor.bytes().filter(|&b| b == b'S').count();
    if count == 0 || count & 1 == 1 {
        return 0;
    }
    let mut p_count = 0;
    let mut s_count = 0;
    let mut res = 1;
    for b in corridor.bytes() {
        if b == b'S' {
            if s_count == 2 {
                res *= i64::from(1 + p_count);
                p_count = 0;
                res %= 1_000_000_007;
                s_count = 1;
            } else {
                s_count += 1;
            }
        } else {
            if s_count == 2 {
                p_count += 1;
            }
        }
    }
    res as _
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
        assert_eq!(number_of_ways("SSPPSPS"), 3);
        assert_eq!(number_of_ways("PPSPSP"), 1);
        assert_eq!(number_of_ways("S"), 0);
    }

    #[test]
    fn test() {}
}
