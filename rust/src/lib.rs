mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_fruit(fruits: &[i32]) -> i32 {
    let mut left = 0;
    let mut counts = std::collections::HashMap::new();
    let mut res = 0;
    for (right, &num) in fruits.iter().enumerate() {
        *counts.entry(num).or_insert(0) += 1;
        if counts.len() < 3 {
            continue;
        }
        let temp = counts.values().sum::<i32>() - 1;
        res = res.max(temp);
        while counts.len() > 2 && left < right {
            let n = fruits[left];
            left += 1;
            let Some(v) = counts.get_mut(&n) else {
                continue;
            };
            *v -= 1;
            if *v == 0 {
                counts.remove(&n);
            }
        }
    }
    res.max(counts.into_values().sum())
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(total_fruit(&[1, 2, 1]), 3);
        debug_assert_eq!(total_fruit(&[0, 1, 2, 2]), 3);
        debug_assert_eq!(total_fruit(&[1, 2, 3, 2, 2]), 4);
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
