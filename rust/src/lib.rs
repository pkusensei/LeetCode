mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_judge(n: i32, trust: &[[i32; 2]]) -> i32 {
    let [mut outs, mut ins] = [0, 1].map(|_| vec![0; n as usize]);
    for v in trust.iter() {
        outs[v[0] as usize - 1] += 1;
        ins[v[1] as usize - 1] += 1;
    }
    outs.into_iter()
        .zip(ins)
        .enumerate()
        .find_map(|(i, (a, b))| {
            if a == 0 && b == n - 1 {
                Some(i as i32 + 1)
            } else {
                None
            }
        })
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_judge(2, &[[1, 2]]), 2);
        debug_assert_eq!(find_judge(3, &[[1, 3], [2, 3]]), 3);
        debug_assert_eq!(find_judge(3, &[[1, 3], [2, 3], [3, 1]]), -1);
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
