mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time(s: &str) -> i32 {
    let n = s.len() as i32;
    let mut left = 0;
    let mut res = n;
    for (idx, b) in (0..).zip(s.bytes()) {
        // Scan from left, for every '1' either
        // 1) remove everything up until current idx => 1+idx
        // 2) single out current idx => left+2*(b-'0')
        left = (left + 2 * i32::from(b - b'0')).min(1 + idx);
        // Consider only removals only from the right
        res = res.min(left + n - idx - 1);
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
        assert_eq!(minimum_time("1100101"), 5);
        assert_eq!(minimum_time("0010"), 2);
    }

    #[test]
    fn test() {}
}
