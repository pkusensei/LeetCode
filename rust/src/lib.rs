mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_submatrix_sum_target(matrix: &[&[i32]], target: i32) -> i32 {
    let (rows, cols) = get_dimensions(matrix);
    let mut prefix = Vec::with_capacity(rows);
    for row in matrix.iter() {
        let mut curr = Vec::with_capacity(cols);
        for &num in row.iter() {
            curr.push(num + curr.last().unwrap_or(&0));
        }
        prefix.push(curr);
    }
    let mut res = 0;
    for left in 0..cols {
        for right in left..cols {
            let mut map = std::collections::HashMap::from([(0, 1)]);
            let mut curr = 0;
            for row in prefix.iter() {
                curr += row[right];
                if left > 0 {
                    curr -= row[left - 1];
                }
                if let Some(v) = map.get(&(curr - target)) {
                    res += v;
                }
                *map.entry(curr).or_insert(0) += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            num_submatrix_sum_target(&[&[0, 1, 0], &[1, 1, 1], &[0, 1, 0]], 0),
            4
        );
        debug_assert_eq!(num_submatrix_sum_target(&[&[1, -1], &[-1, 1]], 0), 5);
        debug_assert_eq!(num_submatrix_sum_target(&[&[904]], 0), 0);
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
