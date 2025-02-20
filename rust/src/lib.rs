mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_product(s: &str) -> i32 {
    backtrack(s.as_bytes(), &mut vec![], &mut vec![])
}

fn backtrack(s: &[u8], s1: &mut Vec<u8>, s2: &mut Vec<u8>) -> i32 {
    match s {
        [] => {
            if is_palindrome(s1.as_slice()) && is_palindrome(s2.as_slice()) {
                return s1.len() as i32 * s2.len() as i32;
            }
            return 0;
        }
        [head, tail @ ..] => {
            let skip = backtrack(tail, s1, s2);
            s1.push(*head);
            let take1 = backtrack(tail, s1, s2);
            s1.pop();
            s2.push(*head);
            let take2 = backtrack(tail, s1, s2);
            s2.pop();
            skip.max(take1).max(take2)
        }
    }
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
        assert_eq!(max_product("leetcodecom"), 9);
        assert_eq!(max_product("bb"), 1);
        assert_eq!(max_product("accbcaxxcxx"), 25);
    }

    #[test]
    fn test() {}
}
