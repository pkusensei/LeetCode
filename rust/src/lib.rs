mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    s.as_bytes()
        .chunks(k as usize)
        .map(|w| {
            let mut v = String::from_utf8(w.to_vec()).unwrap();
            while v.len() < k as usize {
                v.push(fill);
            }
            v
        })
        .collect()
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
