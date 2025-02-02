mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_time(time: String) -> String {
    let (h, m) = time.split_once(':').unwrap();
    let h = h.as_bytes();
    let mut res = vec![];
    match (h[0], h[1]) {
        (b'?', b'?') => res.extend_from_slice(b"23"),
        (b'?', b'0'..b'4') => res.extend([b'2', h[1]]),
        (b'?', b'4'..=b'9') => res.extend([b'1', h[1]]),
        (b'2', b'?') => res.extend_from_slice(b"23"),
        (_, b'?') => res.extend([h[0], b'9']),
        _ => res.extend_from_slice(h),
    }
    res.push(b':');
    let m = m.as_bytes();
    match (m[0], m[1]) {
        (b'?', b'?') => res.extend_from_slice(b"59"),
        (b'?', b'0'..=b'9') => res.extend([b'5', m[1]]),
        (_, b'?') => res.extend([m[0], b'9']),
        _ => res.extend_from_slice(m),
    }
    String::from_utf8(res).unwrap()
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
    fn basics() {}

    #[test]
    fn test() {}
}
