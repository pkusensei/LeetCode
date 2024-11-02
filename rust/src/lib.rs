mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn num_rescue_boats(people: &mut [i32], limit: i32) -> i32 {
    people.sort_unstable();
    let mut queue: VecDeque<i32> = people.iter().copied().collect();
    let mut res = 0;
    while let Some(end) = queue.pop_back() {
        res += 1;
        if queue.front().is_some_and(|&v| v + end <= limit) {
            queue.pop_front();
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
        debug_assert_eq!(num_rescue_boats(&mut [1, 2], 3), 1);
        debug_assert_eq!(num_rescue_boats(&mut [3, 2, 2, 1], 3), 3);
        debug_assert_eq!(num_rescue_boats(&mut [3, 5, 3, 4], 5), 4);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
