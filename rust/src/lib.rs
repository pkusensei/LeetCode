mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_array(n: i32) -> Vec<i32> {
    solve(n, &mut HashMap::new())
}

fn solve(n: i32, memo: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
    if let Some(v) = memo.get(&n) {
        return v.to_vec();
    }
    let mut res = Vec::with_capacity(n as usize);
    if n == 1 {
        res.push(1);
    } else {
        res.extend(solve((n + 1) / 2, memo).into_iter().map(|v| v * 2 - 1));
        res.extend(solve(n / 2, memo).into_iter().map(|v| v * 2));
    }
    memo.insert(n, res.clone());
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(beautiful_array(4), [1, 3, 2, 4]);
        debug_assert_eq!(beautiful_array(5), [1, 5, 3, 2, 4]);
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
