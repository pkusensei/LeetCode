mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    let mut idx = 0;
    while idx <= n {
        let num = to_num(&cells);
        if let Some(prev) = map.get(&num) {
            let period = idx - prev; // is 14
            let count = (n - idx) / period;
            idx += count * period - 1; // no idea why -1 is needed
            map.clear();
        } else {
            if idx == n {
                return cells;
            }
            map.insert(num, idx);
            let mut next = cells.clone();
            (next[0], next[7]) = (0, 0);
            for i in 1..=6 {
                next[i] = (cells[i - 1] == cells[i + 1]).into();
            }
            cells = next;
        }
        idx += 1;
    }
    cells
}

fn to_num(bits: &[i32]) -> u8 {
    let mut res = 0;
    for (i, &b) in bits.iter().enumerate() {
        if b > 0 {
            res |= 1 << i;
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
        debug_assert_eq!(
            prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
            [0, 0, 1, 1, 0, 0, 0, 0]
        );
        debug_assert_eq!(
            prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
            [0, 0, 1, 1, 1, 1, 1, 0]
        );
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
