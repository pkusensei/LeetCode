mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum(nums1: &[i32], nums2: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let (n1, n2) = (nums1.len(), nums2.len());
    // (dfs(nums1, nums2, 0, 0, 0, 0, &mut vec![vec![-1; n2]; n1]) % MOD) as _
    let [mut res, mut sum1, mut sum2] = [0; 3];
    let [mut i1, mut i2] = [0, 0];
    while i1 < n1 && i2 < n2 {
        match nums1[i1].cmp(&nums2[i2]) {
            std::cmp::Ordering::Less => {
                sum1 += i64::from(nums1[i1]);
                i1 += 1;
            }
            std::cmp::Ordering::Equal => {
                res += i64::from(nums1[i1]) + sum1.max(sum2);
                sum1 = 0;
                sum2 = 0;
                i1 += 1;
                i2 += 1;
            }
            std::cmp::Ordering::Greater => {
                sum2 += i64::from(nums2[i2]);
                i2 += 1;
            }
        }
    }
    if i1 < n1 {
        sum1 += nums1[i1..].iter().map(|&v| i64::from(v)).sum::<i64>();
    }
    if i2 < n2 {
        sum2 += nums2[i2..].iter().map(|&v| i64::from(v)).sum::<i64>();
    }
    res += sum1.max(sum2);
    (res % MOD) as _
}

// out of memory
fn dfs(
    nums1: &[i32],
    nums2: &[i32],
    i1: usize,
    i2: usize,
    sum1: i64,
    sum2: i64,
    memo: &mut [Vec<i64>],
) -> i64 {
    if i1 >= nums1.len() || i2 >= nums2.len() {
        return (sum1 + nums1.iter().skip(i1).map(|&v| i64::from(v)).sum::<i64>())
            .max(sum2 + nums2.iter().skip(i2).map(|&v| i64::from(v)).sum::<i64>());
    }
    if memo[i1][i2] > -1 {
        return memo[i1][i2];
    }
    let res = match nums1[i1].cmp(&nums2[i2]) {
        std::cmp::Ordering::Less => dfs(
            nums1,
            nums2,
            1 + i1,
            i2,
            sum1 + i64::from(nums1[i1]),
            sum2,
            memo,
        ),
        std::cmp::Ordering::Equal => {
            sum1.max(sum2) + i64::from(nums1[i1]) + dfs(nums1, nums2, 1 + i1, 1 + i2, 0, 0, memo)
        }
        std::cmp::Ordering::Greater => dfs(
            nums1,
            nums2,
            i1,
            1 + i2,
            sum1,
            sum2 + i64::from(nums2[i2]),
            memo,
        ),
    };
    memo[i1][i2] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_sum(&[2, 4, 5, 8, 10], &[4, 6, 8, 9]), 30);
        assert_eq!(max_sum(&[1, 3, 5, 7, 9], &[3, 5, 100]), 109);
        assert_eq!(max_sum(&[1, 2, 3, 4, 5], &[6, 7, 8, 9, 10]), 40);
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
