mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn cal_points(operations: &[&str]) -> i32 {
    let mut stack = vec![];
    for s in operations.iter() {
        match *s {
            "D" => stack.push(2 * stack.last().unwrap()),
            "C" => {
                stack.pop();
            }
            "+" => {
                let n = stack.len();
                let v = stack[n - 2] + stack[n - 1];
                stack.push(v);
            }
            _ => stack.push(s.parse().unwrap()),
        }
    }
    stack.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(cal_points(&["5", "2", "C", "D", "+"]), 30);
        debug_assert_eq!(cal_points(&["5", "-2", "4", "C", "D", "9", "+", "+"]), 27);
        debug_assert_eq!(cal_points(&["1", "C"]), 0);
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
