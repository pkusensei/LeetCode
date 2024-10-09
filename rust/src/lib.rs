mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_add_to_make_valid(s: &str) -> i32 {
    // let mut stack = vec![];
    // for b in s.bytes() {
    //     if b == b'(' {
    //         stack.push(b);
    //     } else if b == b')' && stack.last().is_some_and(|&v| v == b'(') {
    //         stack.pop();
    //     } else {
    //         stack.push(b')');
    //     }
    // }
    // stack.len() as _
    let (mut open, mut close) = (0, 0);
    for b in s.bytes() {
        if b == b'(' {
            open += 1
        } else if open > 0 {
            open -= 1 // ')'
        } else {
            close += 1
        }
    }
    open + close
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_add_to_make_valid(&"())"), 1);
        debug_assert_eq!(min_add_to_make_valid(&"((("), 3);
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
