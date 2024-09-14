mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn integer_replacement(n: i32) -> i32 {
    let mut map = HashMap::new();
    solve(&mut map, i64::from(n))
}

fn solve(map: &mut HashMap<i64, i32>, n: i64) -> i32 {
    if n == 1 {
        return 0;
    }
    if let Some(&v) = map.get(&n) {
        return v;
    }
    let res;
    if n & 1 == 0 {
        res = 1 + solve(map, n / 2);
        map.insert(n, res);
    } else {
        res = 1 + solve(map, n + 1).min(solve(map, n - 1));
        map.insert(n, res);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(integer_replacement(8), 3);
        debug_assert_eq!(integer_replacement(7), 4);
        debug_assert_eq!(integer_replacement(4), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(integer_replacement(2147483647), 32);
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
