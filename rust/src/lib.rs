mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn repeated_character(s: String) -> char {
    let mut mask = 0;
    for ch in s.chars() {
        let i = usize::from(ch as u8 - b'a');
        if (mask >> i) & 1 == 1 {
            return ch;
        }
        mask |= 1 << i
    }
    '\0'
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
