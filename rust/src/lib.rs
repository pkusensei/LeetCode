mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_ways(nums: &[i32]) -> i32 {
    let table = build(nums.len());
    dfs(nums, &table) as i32 - 1
}

const MOD: i64 = 1_000_000_007;

fn dfs(nums: &[i32], table: &[Vec<i64>]) -> i64 {
    let n = nums.len();
    if n < 3 {
        return 1;
    }
    let root = nums[0];
    let [left, right] = nums
        .iter()
        .fold([vec![], vec![]], |[mut left, mut right], &num| {
            match num.cmp(&root) {
                std::cmp::Ordering::Less => left.push(num),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => right.push(num),
            };
            [left, right]
        });
    (dfs(&left, table) * dfs(&right, table) % MOD) * table[n - 1][left.len()] % MOD
}

fn build(n: usize) -> Vec<Vec<i64>> {
    let mut res = vec![vec![0; n]; n];
    for (i, row) in res.iter_mut().enumerate() {
        row[0] = 1;
        row[i] = 1;
    }
    for row in 2..n {
        for col in 1..row {
            res[row][col] = (res[row - 1][col - 1] + res[row - 1][col]) % MOD;
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
        assert_eq!(num_of_ways(&[2, 1, 3]), 1);
        assert_eq!(num_of_ways(&[3, 4, 5, 1, 2]), 5);
        assert_eq!(num_of_ways(&[1, 2, 3]), 0);
    }

    #[test]
    fn test() {
        assert_eq!(num_of_ways(&[3, 1, 2, 5, 4, 6]), 19);
        assert_eq!(
            num_of_ways(&[9, 4, 2, 1, 3, 6, 5, 7, 8, 14, 11, 10, 12, 13, 16, 15, 17, 18]),
            216212978
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
