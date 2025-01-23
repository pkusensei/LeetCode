mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    let mut rank = vec![vec![0; n as usize]; n as usize];
    for (r, pref) in preferences.iter().enumerate() {
        for (c, &p) in pref.iter().enumerate() {
            rank[r][p as usize] = n - c as i32;
        }
    }
    let mut res = 0;
    let pairs = pairs.into_iter().fold(vec![0; n as usize], |mut acc, p| {
        acc[p[0] as usize] = p[1] as usize;
        acc[p[1] as usize] = p[0] as usize;
        acc
    });
    for (x, &y) in pairs.iter().enumerate() {
        for (u, &r) in rank[x].iter().enumerate() {
            let v = pairs[u];
            if r > rank[x][y] && rank[u][x] > rank[u][v] {
                res += 1;
                break;
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
