mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let mut count = num2.count_ones();
    let mut bits = vec![];
    let mut x = num1;
    while x > 0 {
        bits.push(x & 1);
        x >>= 1;
    }
    bits.reverse();
    for bit in bits.iter_mut() {
        if (*bit) == 1 && count > 0 {
            (*bit) = -1;
            count -= 1;
        }
    }
    for bit in bits.iter_mut().rev() {
        if *bit == 0 && count > 0 {
            (*bit) = 1;
            count -= 1;
        }
    }
    while count > 0 {
        bits.insert(0, 1);
        count -= 1;
    }
    let num = bits
        .into_iter()
        .fold(0, |acc, bit| (acc << 1) | i32::from(bit == 1));
    num ^ num1
}

fn solve(num1: i32, num2: i32) -> i32 {
    // build res as close to num1 as possible
    let mut res = 0;
    let count = num2.count_ones();
    let mut set_bits = 0;
    let mut curr_bit = 31;
    fn is_set(x: i32, bit: i32) -> bool {
        (x & (1 << bit)) != 0
    }
    fn set(x: i32, bit: i32) -> i32 {
        x | (1 << bit)
    }
    while set_bits < count {
        // case 1) curr_bit is set in num1
        // case 2) all remaining bits must be set
        if is_set(num1, curr_bit) || count - set_bits > curr_bit as u32 {
            res = set(res, curr_bit);
            set_bits += 1;
        }
        curr_bit -= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(minimize_xor(3, 5), 3);
        assert_eq!(minimize_xor(1, 12), 3);

        assert_eq!(solve(3, 5), 3);
        assert_eq!(solve(1, 12), 3);
    }

    #[test]
    fn test() {
        assert_eq!(minimize_xor(65, 84), 67);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
