mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn k_concatenation_max_sum(arr: &[i32], k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;

    // Kadane's algo for k==1
    let kadane = {
        let mut curr = 0;
        let mut max = i64::MIN;
        for num in arr.iter().map(|&v| i64::from(v)) {
            curr = num.max(curr + num);
            max = max.max(curr);
        }
        max
    };
    let mut prefix = i64::MIN; // prefix max
    let mut sum = 0;
    for num in arr.iter().map(|&v| i64::from(v)) {
        sum += num;
        prefix = prefix.max(sum);
    }
    sum = 0;
    let mut suffix = i64::MIN; // suffix max
    for num in arr.iter().rev().map(|&v| i64::from(v)) {
        sum += num;
        suffix = suffix.max(sum);
    }
    let k_sum = |c| (0..c).fold(0, |acc, _| acc + sum);
    // max is
    // - Kadanes for k==1
    // - sum * k
    // - prefix_max + suffix_max
    // - prefix_max + suffix_max + sum*(k-2) when k>1
    let mut res = kadane.max(k_sum(k));
    if k > 1 {
        res = res.max(prefix + suffix);
        res = res.max((prefix + suffix) + k_sum(k - 2));
    }
    (res.max(0) % MOD) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(k_concatenation_max_sum(&[1, 2], 3), 9);
        assert_eq!(k_concatenation_max_sum(&[1, -2, 1], 5), 2);
        assert_eq!(k_concatenation_max_sum(&[-1, -2], 7), 0);
    }

    #[test]
    fn test() {
        assert_eq!(k_concatenation_max_sum(&[1, 2], 1), 3);
        assert_eq!(
            k_concatenation_max_sum(
                &[-9, 13, 4, -16, -12, -16, 3, -7, 5, -16, 16, 8, -1, -13, 15, 3],
                6
            ),
            36
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
