mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn last_remaining(n: i32) -> i32 {
    let mut forward = true;
    let mut n = n as usize;
    let (mut head, mut step) = (1, 1);
    while n > 1 {
        if forward || n & 1 == 1 {
            head += step;
        }
        n /= 2;
        step *= 2;
        forward = !forward
    }
    head
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(last_remaining(9), 6);
        debug_assert_eq!(last_remaining(1), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
