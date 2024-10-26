mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_profit_assignment(difficulty: &[i32], profit: &[i32], worker: &[i32]) -> i32 {
    let mut map = difficulty
        .iter()
        .zip(profit.iter())
        .map(|(&d, &p)| (d, p))
        .fold(std::collections::BTreeMap::new(), |mut acc, (k, v)| {
            if acc.get(&k).is_none_or(|&n| n < v) {
                acc.insert(k, v);
            }
            acc
        });
    let mut prev = 0;
    for v in map.values_mut() {
        if *v > prev {
            prev = *v;
        }
        *v = (*v).max(prev);
    }
    let mut res = 0;
    for &w in worker.iter() {
        res += map.range(..=w).next_back().map(|(_, &v)| v).unwrap_or(0);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            max_profit_assignment(&[2, 4, 6, 8, 10], &[10, 20, 30, 40, 50], &[4, 5, 6, 7]),
            100
        );
        debug_assert_eq!(
            max_profit_assignment(&[85, 47, 57], &[24, 66, 99], &[40, 25, 25]),
            0
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            max_profit_assignment(
                &[66, 1, 28, 73, 53, 35, 45, 60, 100, 44, 59, 94, 27, 88, 7, 18, 83, 18, 72, 63],
                &[66, 20, 84, 81, 56, 40, 37, 82, 53, 45, 43, 96, 67, 27, 12, 54, 98, 19, 47, 77],
                &[61, 33, 68, 38, 63, 45, 1, 10, 53, 23, 66, 70, 14, 51, 94, 18, 28, 78, 100, 16]
            ),
            1392
        );
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
