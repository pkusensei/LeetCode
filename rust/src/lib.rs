mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn capitalize_title(title: String) -> String {
    title
        .split_ascii_whitespace()
        .map(|s| {
            let s = s.to_ascii_lowercase();
            if s.len() > 2 {
                let mut v = s[..1].to_uppercase();
                v.push_str(&s[1..]);
                v
            } else {
                s
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
