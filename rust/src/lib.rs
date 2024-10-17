mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost_climbing_stairs(cost: &[i32]) -> i32 {
    let (mut c1, mut c2) = (cost[0], cost[1]);
    for &num in cost.iter().skip(2) {
        let curr = num + c1.min(c2);
        c1 = c2;
        c2 = curr
    }
    c1.min(c2)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_cost_climbing_stairs(&[10, 15, 20]), 15);
        debug_assert_eq!(
            min_cost_climbing_stairs(&[1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
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
