mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_good_base(n: &str) -> String {
    let n: i64 = n.parse().unwrap();
    // n = k^0 + k^1 + k^2 + .. + k^(m-1)
    // n = (1 - k^m)/(1-k)
    // n ~= k^(m-1)
    // k ~= n^(1/(m-1))
    let len = n.ilog2() + 1; // max(m)
    for m in (2..=len).rev() {
        let k = (n as f64).powf(1.0 / (m as f64 - 1.0)) as i64;
        // n-1 = k(1 + k^0 + k^1 + .. + k^(m-2))
        // n-1 = k* (1-k^(m-1))/(1-k)
        // (n-1)*(1-k)/k = (1 - k^(m-1))
        if (n - 1) % k == 0 && (n - 1) / k * (1 - k) == 1 - k.pow(m - 1) {
            return k.to_string();
        }
    }
    // deals with `1-** / 1-**` in (1 - k^m)/(1-k)
    (n - 1).to_string()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(smallest_good_base("13"), "3");
        debug_assert_eq!(smallest_good_base("4681"), "8");
        debug_assert_eq!(
            smallest_good_base("1000000000000000000"),
            "999999999999999999"
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
}
