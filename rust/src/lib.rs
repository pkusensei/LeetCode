mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
    let Some(i1) =
        arr.windows(2)
            .enumerate()
            .rev()
            .find_map(|(i, w)| if w[0] > w[1] { Some(i) } else { None })
    else {
        return arr;
    };
        let mut i2 = 1 + i1;
        let n = arr.len();
        for i in 1 + i1..n {
            if (1 + arr[i2]..arr[i1]).contains(&arr[i]) {
                i2 = i
            }
        }
        arr.swap(i1, i2);
    arr
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(prev_perm_opt1(vec![3, 2, 1]), [3, 1, 2]);
        debug_assert_eq!(prev_perm_opt1(vec![1, 1, 5]), [1, 1, 5]);
        debug_assert_eq!(prev_perm_opt1(vec![1, 9, 4, 6, 7]), [1, 7, 4, 6, 9]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(prev_perm_opt1(vec![3, 1, 1, 3]), [1, 3, 1, 3]);
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
