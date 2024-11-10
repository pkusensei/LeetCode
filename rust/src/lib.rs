mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    tokens.sort_unstable();
    let mut tokens = VecDeque::from(tokens);
    let mut curr = 0;
    let mut res = 0;
    while !tokens.is_empty() {
        while tokens.front().is_some_and(|&v| v <= power) {
            let Some(v) = tokens.pop_front() else {
                break;
            };
            power -= v;
            curr += 1;
        }
        if curr == 0 {
            break;
        }
        res = res.max(curr);
        if let Some(v) = tokens.pop_back() {
            curr -= 1;
            power += v;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(bag_of_tokens_score(vec![100], 50), 0);
        debug_assert_eq!(bag_of_tokens_score(vec![200, 100], 150), 1);
        debug_assert_eq!(bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(bag_of_tokens_score(vec![33, 4, 28, 24, 96], 35), 3);
    }

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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
