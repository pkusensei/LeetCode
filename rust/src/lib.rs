mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn str_without3a3b(a: i32, b: i32) -> String {
    let mut heap = std::collections::BinaryHeap::from([(a, b'a'), (b, b'b')]);
    let mut res = Vec::with_capacity((a + b) as usize);
    while let Some((c1, b1)) = heap.pop() {
        if res.ends_with(&[b1; 2]) {
            let Some((c2, b2)) = heap.pop() else {
                break;
            };
            res.push(b2);
            if c2 > 1 {
                heap.push((c2 - 1, b2));
            }
            heap.push((c1, b1));
        } else {
            res.push(b1);
            if c1 > 1 {
                heap.push((c1 - 1, b1));
            }
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(str_without3a3b(1, 2), "bba");
        debug_assert_eq!(str_without3a3b(4, 1), "aabaa");
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
