mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn partition_string(s: String) -> i32 {
    let mut mask = 0;
    let mut res = 1;
    for b in s.bytes() {
        let bit = i32::from(b - b'a');
        if (mask >> bit) & 1 == 0 {
            mask |= 1 << bit;
        } else {
            mask = 1 << bit;
            res += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
