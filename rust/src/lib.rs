mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn discount_prices(sentence: &str, discount: i32) -> String {
    sentence
        .split_whitespace()
        .map(|s| {
            if let Some(s) = s.strip_prefix('$') {
                if !s.is_empty() && s.bytes().all(|v| v.is_ascii_digit()) {
                    let num: f64 = s.parse().unwrap();
                    let d = 1.0 - f64::from(discount) / 100.0;
                    return format!("${:.2}", num * d);
                }
            }
            s.to_string()
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
    fn test() {
        assert_eq!(discount_prices("$1e9", 50), "$1e9");
    }
}
