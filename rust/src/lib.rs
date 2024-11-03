mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn at_most_n_given_digit_set(digits: &[&str], n: i32) -> i32 {
    let digits: Vec<i32> = digits.iter().map(|s| s.parse().unwrap_or(1)).collect();
    let d_count = digits.len();
    let n_bytes = n.to_string().into_bytes();
    let n_len = n_bytes.len();
    let mut dp = vec![0; 1 + n_len];
    dp[n_len] = 1;
    for (idx, b) in n_bytes.into_iter().enumerate().rev() {
        let digit = i32::from(b - b'0');
        for &d in digits.iter() {
            match d.cmp(&digit) {
                // (d in n) < (digit option)
                // take all combos on digits to the right [1+idx..]
                // count ** (n_len - 1-idx)
                std::cmp::Ordering::Less => dp[idx] += d_count.pow((n_len - idx - 1) as _),
                // d == option
                // Choices on [1+idx..] is limited
                std::cmp::Ordering::Equal => dp[idx] += dp[1 + idx],
                std::cmp::Ordering::Greater => (),
            }
        }
    }
    // add up nums with less digits than n
    ((1..n_len).map(|v| d_count.pow(v as _)).sum::<usize>() + dp[0]) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(at_most_n_given_digit_set(&["1", "3", "5", "7"], 100), 20);
        debug_assert_eq!(
            at_most_n_given_digit_set(&["1", "4", "9"], 1_000_000_000),
            29523
        );
        debug_assert_eq!(at_most_n_given_digit_set(&["7"], 8), 1);
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
