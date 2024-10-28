mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn backspace_compare(s: &str, t: &str) -> bool {
    process(s) == process(t)
}

fn process(s: &str) -> Vec<u8> {
    let mut stack = vec![];
    for b in s.bytes() {
        if b == b'#' {
            stack.pop();
        } else {
            stack.push(b);
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(backspace_compare("ab#c", "ad#c"));
        debug_assert!(backspace_compare("ab##", "c#d#",));
        debug_assert!(!backspace_compare("a#c", "b",))
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
