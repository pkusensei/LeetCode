mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn parse_bool_expr(expression: &str) -> bool {
    let mut stack = vec![];
    for b in expression.bytes() {
        match b {
            b'&' | b'|' | b'!' | b't' | b'f' => stack.push(b),
            b')' => {
                let mut vals = vec![];
                while stack.last().is_some_and(|v| v.is_ascii_lowercase()) {
                    vals.push(stack.pop().unwrap());
                }
                let Some(op) = stack.pop() else {
                    break;
                };
                match op {
                    b'!' => {
                        debug_assert_eq!(vals.len(), 1);
                        stack.push(if vals[0] == b't' { b'f' } else { b't' });
                    }
                    b'&' => {
                        stack.push(if vals.contains(&b'f') { b'f' } else { b't' });
                    }
                    b'|' => {
                        stack.push(if vals.contains(&b't') { b't' } else { b'f' });
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    debug_assert_eq!(stack.len(), 1);
    stack[0] == b't'
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!parse_bool_expr("&(|(f))"));
        debug_assert!(parse_bool_expr("|(f,f,f,t)"));
        debug_assert!(parse_bool_expr("!(&(f,t))"));
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
