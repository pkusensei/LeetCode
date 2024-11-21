mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn two_city_sched_cost(costs: &mut [[i32; 2]]) -> i32 {
    costs.sort_unstable_by_key(|v| v[0] - v[1]);
    let n = costs.len();
    costs[..n / 2]
        .iter()
        .map(|v| v[0])
        .chain(costs[n / 2..].iter().map(|v| v[1]))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            two_city_sched_cost(&mut [[10, 20], [30, 200], [400, 50], [30, 20]]),
            110
        );
        debug_assert_eq!(
            two_city_sched_cost(&mut [
                [259, 770],
                [448, 54],
                [926, 667],
                [184, 139],
                [840, 118],
                [577, 469]
            ]),
            1859
        );
        debug_assert_eq!(
            two_city_sched_cost(&mut [
                [515, 563],
                [451, 713],
                [537, 709],
                [343, 819],
                [855, 779],
                [457, 60],
                [650, 359],
                [631, 42]
            ]),
            3086
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
