mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_time(time: String) -> i32 {
    let (h, m) = time.split_once(':').unwrap();
    let h = match [h.as_bytes()[0], h.as_bytes()[1]] {
        [b'?', b'?'] => 24,
        [b'0' | b'1', b'?'] => 10,
        [b'2', b'?'] => 4,
        [b'?', b'0'..=b'3'] => 3,
        [b'?', _] => 2,
        _ => 1,
    };
    let m = match [m.as_bytes()[0], m.as_bytes()[1]] {
        [b'?', b'?'] => 60,
        [b'0'..=b'5', b'?'] => 10,
        [b'?', b'0'..=b'9'] => 6,
        _ => 1,
    };
    h * m
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
