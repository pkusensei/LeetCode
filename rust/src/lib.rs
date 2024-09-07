mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn bulb_switch(n: i32) -> i32 {
    // The ith bulb has to be touched TWICE to stay on
    // First time sqrt(i) to turn it off
    // Then i to turn it back on
    (n as f64).sqrt().trunc() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(bulb_switch(3), 1);
        debug_assert_eq!(bulb_switch(0), 0);
        debug_assert_eq!(bulb_switch(1), 1);
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
