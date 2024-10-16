mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn monotone_increasing_digits(n: i32) -> i32 {
    let mut num = to_digits(n);
    // 332 => 329 => 299
    while let Some(idx) =
        num.windows(2)
            .enumerate()
            .rev()
            .find_map(|(i, w)| if w[0] > w[1] { Some(i) } else { None })
    {
        num[idx] -= 1;
        num[idx + 1..].fill(9);
    }
    from_digits(num)
}

fn from_digits(num: Vec<i32>) -> i32 {
    num.into_iter().fold(0, |acc, d| acc * 10 + d)
}

fn to_digits(mut n: i32) -> Vec<i32> {
    let mut res = vec![];
    while n > 0 {
        res.push(n % 10);
        n /= 10;
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(monotone_increasing_digits(10), 9);
        debug_assert_eq!(monotone_increasing_digits(1234), 1234);
        debug_assert_eq!(monotone_increasing_digits(332), 299);
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
