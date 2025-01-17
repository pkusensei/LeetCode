mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_distance(houses: &mut [i32], k: i32) -> i32 {
    houses.sort_unstable();
    let n = houses.len();
    dfs(houses, k, 0, &mut vec![vec![None; n]; 1 + k as usize]).unwrap_or(cost(houses))
}

fn dfs(houses: &[i32], k: i32, idx: usize, memo: &mut [Vec<Option<i32>>]) -> Option<i32> {
    let n = houses.len();
    if k == 0 && idx >= n {
        return Some(0);
    }
    if k == 0 || idx >= n {
        return None;
    }
    if let Some(v) = memo[k as usize][idx] {
        return if v == -1 { None } else { Some(v) };
    }
    let mut res = i32::MAX;
    for end in idx..n {
        if let Some(v) = dfs(houses, k - 1, 1 + end, memo) {
            let c = cost(&houses[idx..=end]);
            res = res.min(v + c);
        }
    }
    if res == i32::MAX {
        memo[k as usize][idx] = Some(-1);
        None
    } else {
        memo[k as usize][idx] = Some(res);
        Some(res)
    }
}

fn cost(vals: &[i32]) -> i32 {
    let n = vals.len();
    if n <= 1 {
        0
    } else {
        let [mut left, mut right] = [0, n - 1];
        let mut res = 0;
        while left < right {
            res += vals[right] - vals[left];
            left += 1;
            right -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_distance(&mut [1, 4, 8, 10, 20], 3), 5);
        assert_eq!(min_distance(&mut [2, 3, 5, 12, 18], 2), 9);
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
