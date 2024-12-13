mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    'out: for x in 1..=1000 {
        let [mut left, mut right] = [1, 1000];
        while left <= right {
            let mid = left + (right - left) / 2;
            match customfunction.f(x, mid).cmp(&z) {
                std::cmp::Ordering::Less => left = 1 + mid,
                std::cmp::Ordering::Equal => {
                    res.push(vec![x, mid]);
                    continue 'out;
                }
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }
    }
    res
}

pub struct CustomFunction;
impl CustomFunction {
    pub fn f(&self, x: i32, y: i32) -> i32 {
        42
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
