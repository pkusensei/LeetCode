mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_triplets(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut prefix = Vec::with_capacity(n);
    for &num in arr.iter() {
        prefix.push(num ^ prefix.last().unwrap_or(&0));
    }
    let mut res = 0;
    for left in 0..n - 1 {
        for right in 1 + left..n {
            let p = if left > 0 {
                prefix[right] ^ prefix[left - 1]
            } else {
                prefix[right]
            };
            if p == 0 {
                res += right - left;
            }
        }
    }
    res as _
}

fn with_map(arr: &[i32]) -> i32 {
    let mut map = std::collections::HashMap::from([(0, vec![-1])]);
    let mut curr = 0;
    let mut res = 0;
    for (idx, &num) in arr.iter().enumerate() {
        curr ^= num;
        if let Some(prev) = map.get(&curr) {
            res += prev.iter().map(|p| idx as i32 - p - 1).sum::<i32>();
        }
        map.entry(curr).or_default().push(idx as i32);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(count_triplets(&[2, 3, 1, 6, 7]), 4);
        assert_eq!(count_triplets(&[1, 1, 1, 1, 1]), 10);

        assert_eq!(with_map(&[2, 3, 1, 6, 7]), 4);
        assert_eq!(with_map(&[1, 1, 1, 1, 1]), 10);
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
