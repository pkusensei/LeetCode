mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct MajorityChecker {
    data: HashMap<i32, Vec<i32>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let data = arr.into_iter().enumerate().fold(
            HashMap::<i32, Vec<i32>>::new(),
            |mut acc, (i, num)| {
                acc.entry(num).or_default().push(i as i32);
                acc
            },
        );
        Self { data }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        for (&k, val) in self.data.iter() {
            let li = val.partition_point(|&i| i < left);
            let ri = val.partition_point(|&i| i <= right);
            if li < val.len() && li < ri {
                let count = ri - li;
                if count >= threshold as usize {
                    return k;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mc = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
        assert_eq!(mc.query(0, 5, 4), 1); // return 1
        assert_eq!(mc.query(0, 3, 3), -1); // return -1
        assert_eq!(mc.query(2, 3, 2), 2); // return 2
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
