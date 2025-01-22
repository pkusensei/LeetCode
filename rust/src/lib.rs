mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_routes(locations: &[i32], start: i32, finish: i32, fuel: i32) -> i32 {
    let n = locations.len();
    dfs(
        locations,
        finish as _,
        start as _,
        fuel,
        &mut vec![vec![-1; 1 + fuel as usize]; n],
    )
}

fn dfs(nums: &[i32], finish: usize, curr: usize, fuel: i32, memo: &mut [Vec<i32>]) -> i32 {
    if fuel < 0 {
        return 0;
    }
    if memo[curr][fuel as usize] > -1 {
        return memo[curr][fuel as usize];
    }
    let mut res = i32::from(curr == finish);
    for (idx, num) in nums.iter().enumerate() {
        if idx == curr {
            continue;
        }
        res += dfs(nums, finish, idx, fuel - (nums[curr] - num).abs(), memo);
        res %= 1_000_000_007;
    }
    memo[curr][fuel as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(count_routes(&[2, 3, 6, 8, 4], 1, 3, 5), 4);
        assert_eq!(count_routes(&[4, 3, 1], 1, 0, 6), 5);
        assert_eq!(count_routes(&[5, 2, 1], 0, 2, 3), 0);
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
