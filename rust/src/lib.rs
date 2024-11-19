mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_domino_rotations(tops: &[i32], bottoms: &[i32]) -> i32 {
    let n = tops.len();
    if n != bottoms.len() {
        return -1;
    }
    let counts =
        tops.iter()
            .chain(bottoms.iter())
            .fold(std::collections::HashMap::new(), |mut acc, &v| {
                *acc.entry(v).or_insert(0) += 1;
                acc
            });
    let mut res = None::<i32>;
    for candid in counts
        .iter()
        .filter_map(|(&num, &count)| if count >= n { Some(num) } else { None })
    {
        let (mut t1, mut t2) = (0, 0);
        for (&a, &b) in tops.iter().zip(bottoms.iter()) {
            if a != candid && b != candid {
                return -1;
            } else if a != candid && b == candid {
                t1 += 1;
            } else if b != candid && a == candid {
                t2 += 1;
            }
        }
        if let Some(ref mut v) = res {
            *v = (*v).min(t1).min(t2)
        } else {
            res = Some(t1.min(t2))
        }
    }
    res.unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            min_domino_rotations(&[2, 1, 2, 4, 2, 2], &[5, 2, 6, 2, 3, 2]),
            2
        );
        debug_assert_eq!(min_domino_rotations(&[3, 5, 1, 2, 3], &[3, 6, 3, 3, 4]), -1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            min_domino_rotations(&[1, 2, 1, 1, 1, 2, 2, 2], &[2, 1, 2, 2, 2, 2, 2, 2]),
            1
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
