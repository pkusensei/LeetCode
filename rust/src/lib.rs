mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_radius(houses: &[i32], heaters: &mut [i32]) -> i32 {
    heaters.sort_unstable();
    let n = heaters.len();
    let mut res = 0;
    for h in houses.iter() {
        if let Err(i) = heaters.binary_search(h) {
            if i == n {
                res = res.max(h - heaters[n - 1]);
            } else if i == 0 {
                res = res.max(heaters[0] - h);
            } else {
                res = res.max((h - heaters[i - 1]).min(heaters[i] - h));
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
    fn basics() {
        debug_assert_eq!(find_radius(&[1, 2, 3], &mut [2]), 1);
        debug_assert_eq!(find_radius(&[1, 2, 3, 4], &mut [1, 4]), 1);
        debug_assert_eq!(find_radius(&[1, 5], &mut [2]), 3);
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
