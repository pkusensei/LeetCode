mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_make_pali_queries(s: &str, queries: &[[i32; 3]]) -> Vec<bool> {
    let n = s.len();
    let mut prefix: Vec<_> = (0..26).map(|_| Vec::with_capacity(n)).collect();
    for b in s.bytes() {
        for i in 0..26 {
            let last = prefix[i].last().copied().unwrap_or(0);
            prefix[i].push(i32::from(usize::from(b - b'a') == i) + last);
        }
    }
    let mut res = Vec::with_capacity(n);
    for q in queries.iter() {
        let [left, right, k] = [q[0], q[1], q[2]].map(|v| v as usize);
        let odd = prefix
            .iter()
            .map(|p| {
                if left == 0 {
                    p[right]
                } else {
                    p[right] - p[left - 1]
                }
            })
            .filter(|&v| v & 1 == 1)
            .count();
        res.push(odd <= 1 + 2 * k);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            can_make_pali_queries(
                "abcda",
                &[[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]]
            ),
            [true, false, false, true, true]
        );
        assert_eq!(
            can_make_pali_queries("lyb", &[[0, 1, 0], [2, 2, 1]]),
            [false, true]
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
