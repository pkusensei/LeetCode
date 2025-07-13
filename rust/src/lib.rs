mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn process_str(s: &str, mut k: i64) -> char {
    let mut len = 0_i64;
    for b in s.bytes() {
        match b {
            b'*' => len = (len - 1).max(0),
            b'#' => len *= 2,
            b'%' => (),
            _ => len += 1,
        }
    }
    if k >= len {
        return '.';
    }
    for ch in s.chars().rev() {
        match ch {
            '*' => len += 1,
            '#' => {
                len /= 2;
                if k >= len {
                    k -= len;
                }
            }
            '%' => k = len - k - 1,
            _ => {
                len -= 1;
                if k == len {
                    return ch;
                }
            }
        }
    }
    '.'
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(process_str("a#b%*", 1), 'a');
        assert_eq!(process_str("cd%#*#", 3), 'd');
    }

    #[test]
    fn test() {
        assert_eq!(process_str("jio#*g", 1), 'i');
    }
}
