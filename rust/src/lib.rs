mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn longest_wpi(hours: &mut [i32]) -> i32 {
    // for v in hours.iter_mut() {
    //     *v = if *v > 8 { 1 } else { -1 };
    // }
    // let n = hours.len();
    // let mut prefix = Vec::with_capacity(1 + n);
    // prefix.push(0);
    // for v in hours.iter() {
    //     prefix.push(v + prefix.last().unwrap_or(&0));
    // }
    // let mut res = 0;
    // for i1 in 1..=n {
    //     for i2 in 0..i1 {
    //         if prefix[i2] < prefix[i1] {
    //             res = res.max(i1 - i2);
    //             break;
    //         }
    //     }
    // }
    // res as i32
    let mut seen = HashMap::new();
    let mut res = 0;
    let mut sum = 0;
    for (idx, &v) in hours.iter().enumerate() {
        sum += if v > 8 { 1 } else { -1 };
        if sum > 0 {
            res = 1 + idx
        } else {
            seen.entry(sum).or_insert(idx);
            if let Some(prev) = seen.get(&(sum - 1)) {
                res = res.max(idx - prev);
            }
        }
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_wpi(&mut [9, 9, 6, 0, 6, 6, 9]), 3);
        debug_assert_eq!(longest_wpi(&mut [6, 6, 6]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(longest_wpi(&mut [9, 9, 9]), 3);
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
