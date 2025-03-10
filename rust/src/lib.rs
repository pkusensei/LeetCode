mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_variance(s: &str) -> i32 {
    let count = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = 0;
    for a in 0..26 {
        for b in 0..26 {
            if a == b || count[a] == 0 || count[b] == 0 {
                continue;
            }
            let [ch1, ch2] = [a, b].map(|v| v as u8 + b'a');
            let [mut major, mut minor] = [0, 0];
            let mut rest_minor = count[b];
            for b in s.bytes() {
                if b == ch1 {
                    major += 1;
                }
                if b == ch2 {
                    minor += 1;
                    rest_minor -= 1;
                }
                // only when there is a minor char
                if minor > 0 {
                    res = res.max(major - minor);
                }
                // reset only when a minor exists in the rest
                if major < minor && rest_minor > 0 {
                    major = 0;
                    minor = 0;
                }
            }
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
        assert_eq!(largest_variance("aababbb"), 3);
        assert_eq!(largest_variance("abcde"), 0);
    }

    #[test]
    fn test() {}
}
