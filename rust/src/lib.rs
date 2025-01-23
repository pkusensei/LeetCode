mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations_max_profit(customers: &[i32], boarding_cost: i32, running_cost: i32) -> i32 {
    let mut wait = 0;
    let mut profit = 0;
    let mut res = i32::MIN;
    let mut max = i32::MIN;
    let mut count = 0;
    for num in customers {
        wait += num;
        let temp = (wait - 4).max(0);
        profit += (wait - temp) * boarding_cost - running_cost;
        wait = temp;
        count += 1;
        if profit > max {
            max = profit;
            res = count
        }
    }
    while wait > 0 {
        let temp = (wait - 4).max(0);
        profit += (wait - temp) * boarding_cost - running_cost;
        wait = temp;
        count += 1;
        if profit > max {
            max = profit;
            res = count
        }
    }
    if max <= 0 {
        -1
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_operations_max_profit(&[8, 3], 5, 6), 3);
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
