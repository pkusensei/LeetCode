mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn car_fleet(target: i32, position: &[i32], speed: &[i32]) -> i32 {
    let mut pairs: Vec<_> = position
        .iter()
        .copied()
        .zip(speed.iter().copied())
        .collect();
    pairs.sort_unstable_by_key(|&(pos, _)| std::cmp::Reverse(pos));
    let mut stack = vec![];
    for (pos, speed) in pairs {
        let t = f64::from(target - pos) / f64::from(speed);
        if stack.last().is_some_and(|&v| v >= t) {
            continue; // catches up and stays in fleet
        }
        stack.push(t);
    }
    stack.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(car_fleet(12, &[10, 8, 0, 5, 3], &[2, 4, 1, 1, 3],), 3);
        debug_assert_eq!(car_fleet(10, &[3], &[3],), 1);
        debug_assert_eq!(car_fleet(100, &[0, 2, 4], &[4, 2, 1],), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(car_fleet(10, &[0, 4, 2], &[2, 1, 3]), 1);
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
}
