mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn greatest_letter(s: String) -> String {
    let count = s.bytes().fold([0; 26], |mut acc, b| {
        if b.is_ascii_uppercase() {
            acc[usize::from(b - b'A')] |= 0b01
        } else if b.is_ascii_lowercase() {
            acc[usize::from(b - b'a')] |= 0b10
        }
        acc
    });
    count
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, &v)| {
            if v == 0b11 {
                String::from_utf8(vec![i as u8 + b'A']).ok()
            } else {
                None
            }
        })
        .unwrap_or_default()
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
