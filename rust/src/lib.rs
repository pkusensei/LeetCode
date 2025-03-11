mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn digit_count(num: String) -> bool {
    let count = num.bytes().fold([0; 10], |mut acc, b| {
        acc[usize::from(b - b'0')] += 1;
        acc
    });
    for (idx, b) in num.bytes().enumerate() {
        if b - b'0' != count[idx] {
            return false;
        }
    }
    true
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
