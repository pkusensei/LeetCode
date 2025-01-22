mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&is_water);
    let mut res = vec![vec![0; cols]; rows];
    let mut queue = std::collections::VecDeque::new();
    for (row, r) in is_water.iter().enumerate() {
        for (col, &v) in r.iter().enumerate() {
            if v == 1 {
                queue.push_back([row, col]);
            }
        }
    }
    let mut curr = 0;
    while !queue.is_empty() {
        curr += 1;
        let n = queue.len();
        for _ in 0..n {
            let Some([row, col]) = queue.pop_front() else {
                break;
            };
            for [nr, nc] in neighbors([row, col]).filter(|&[nr, nc]| {
                is_water
                    .get(nr)
                    .is_some_and(|r| r.get(nc).is_some_and(|&v| v == 0))
            }) {
                if res[nr][nc] == 0 {
                    res[nr][nc] = curr;
                    queue.push_back([nr, nc]);
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
