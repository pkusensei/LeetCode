mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn appeal_sum(s: &str) -> i64 {
    let mut chs = [-1; 26]; // prev idx
    let mut res = 0;
    let mut curr = 0;
    for (i, b) in (0..).zip(s.bytes()) {
        curr += i - chs[usize::from(b - b'a')]; // exclude dup
        res += curr;
        chs[usize::from(b - b'a')] = i;
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
        assert_eq!(appeal_sum("abbca"), 28);
        assert_eq!(appeal_sum("code"), 20);
    }

    #[test]
    fn test() {}
}
