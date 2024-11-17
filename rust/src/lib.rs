mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
    num.reverse();
    let mut idx = 0;
    let mut carry = 0;
    while k > 0 {
        if let Some(v) = num.get_mut(idx) {
            let sum = *v + k % 10 + carry;
            carry = sum / 10;
            *v = sum % 10;
        } else {
            let sum = k % 10 + carry;
            carry = sum / 10;
            num.push(sum % 10);
        }
        idx += 1;
        k /= 10;
    }
    while carry > 0 {
        if let Some(v) = num.get_mut(idx) {
            let sum = *v + carry;
            carry = sum / 10;
            *v = sum % 10;
        } else {
            num.push(carry);
            carry = 0;
        }
        idx += 1;
    }
    num.reverse();
    num
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(add_to_array_form(vec![1, 2, 0, 0], 34), [1, 2, 3, 4]);
        debug_assert_eq!(add_to_array_form(vec![2, 7, 4], 181), [4, 5, 5]);
        debug_assert_eq!(add_to_array_form(vec![2, 1, 5], 806), [1, 0, 2, 1]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
