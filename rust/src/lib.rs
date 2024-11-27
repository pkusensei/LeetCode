mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_subsequence(s: &str) -> String {
    let mut count = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut seen = [false; 26];
    let mut stack = vec![];
    for b in s.bytes() {
        let idx = usize::from(b - b'a');
        count[idx] -= 1;
        if seen[idx] {
            continue;
        }
        while stack
            .last()
            .is_some_and(|&v| v >= b && count[usize::from(v - b'a')] > 0)
        {
            let Some(v) = stack.pop() else {
                break;
            };
            seen[usize::from(v - b'a')] = false;
        }
        stack.push(b);
        seen[idx] = true;
    }
    String::from_utf8(stack).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(smallest_subsequence("bcabc"), "abc");
        debug_assert_eq!(smallest_subsequence("cbacdcbc"), "acdb");
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
