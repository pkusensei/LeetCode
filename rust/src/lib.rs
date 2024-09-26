mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn find_maximized_capital(k: i32, mut w: i32, profits: &[i32], capital: &[i32]) -> i32 {
    let mut cp: Vec<_> = capital
        .iter()
        .copied()
        .zip(profits.iter().copied())
        .collect();
    cp.sort_unstable();
    let mut idx = 0;
    let mut pq = BinaryHeap::new();
    for _ in 0..k {
        while cp.get(idx).is_some_and(|v| v.0 <= w) {
            pq.push(cp[idx].1);
            idx += 1
        }
        let Some(v) = pq.pop() else {
            break;
        };
        w += v;
    }
    w
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_maximized_capital(2, 0, &[1, 2, 3], &[0, 1, 1]), 4);
        debug_assert_eq!(find_maximized_capital(3, 0, &[1, 2, 3], &[0, 1, 2]), 6);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_maximized_capital(10, 0, &[1, 2, 3], &[0, 1, 2]), 6);
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
}
