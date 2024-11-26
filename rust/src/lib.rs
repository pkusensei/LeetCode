mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_champion(n: i32, edges: &[[i32; 2]]) -> i32 {
    let mut degs = vec![0; n as usize];
    for e in edges.iter() {
        degs[e[1] as usize] += 1;
    }
    let mut it = degs.into_iter().enumerate().filter(|(_i, v)| *v == 0);
    if let (Some(v), None) = (it.next(), it.next()) {
        v.0 as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_champion(3, &[[0, 1], [1, 2]]), 0);
        debug_assert_eq!(find_champion(4, &[[0, 2], [1, 3], [1, 2]]), -1);
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
