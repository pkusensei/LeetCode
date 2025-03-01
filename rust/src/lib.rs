mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_be_valid(s: String, locked: String) -> bool {
    let mut free = vec![];
    let mut locked_open = vec![];
    for (idx, (a, b)) in s.bytes().zip(locked.bytes()).enumerate() {
        match (a, b) {
            (b'(', b'1') => locked_open.push(idx),
            (b')', b'1') => {
                if locked_open.pop().is_none() && free.pop().is_none() {
                    return false;
                }
            }
            _ => free.push(idx),
        }
    }
    while let Some(open) = locked_open.pop() {
        if !free.pop().is_some_and(|i| i > open) {
            return false;
        }
    }
    free.len() & 1 == 0
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
