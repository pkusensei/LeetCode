mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_students(seats: &[&[char]]) -> i32 {
    let (rows, cols) = get_dimensions(seats);
    let valid: Vec<_> = seats
        .iter()
        .map(|row| {
            row.iter()
                .fold(0, |acc, &ch| acc << 1 | usize::from(ch == '.'))
        })
        .collect();
    let mut prev = vec![-1; 1 << cols];
    prev[0] = 0; // start with a "virtual" row
    for row in 1..=rows {
        let mut curr = vec![-1; 1 << cols];
        for (mask, curr_val) in curr.iter_mut().enumerate() {
            // no '#' seats
            // no adjacent seats
            if mask & valid[row - 1] != mask || mask & (mask >> 1) > 0 {
                continue;
            }
            for (prev_mask, &prev_val) in prev.iter().enumerate() {
                // upper right
                // upper left
                // prev_val is valid state
                if (mask >> 1) & prev_mask == 0 && mask & (prev_mask >> 1) == 0 && prev_val != -1 {
                    *curr_val = (*curr_val).max(prev_val + mask.count_ones() as i32);
                }
            }
        }
        prev = curr;
    }
    prev.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            max_students(&[
                &['#', '.', '#', '#', '.', '#'],
                &['.', '#', '#', '#', '#', '.'],
                &['#', '.', '#', '#', '.', '#']
            ]),
            4
        );
        assert_eq!(
            max_students(&[
                &['.', '#'],
                &['#', '#'],
                &['#', '.'],
                &['#', '#'],
                &['.', '#']
            ]),
            3
        );
        assert_eq!(
            max_students(&[
                &['#', '.', '.', '.', '#'],
                &['.', '#', '.', '#', '.'],
                &['.', '.', '#', '.', '.'],
                &['.', '#', '.', '#', '.'],
                &['#', '.', '.', '.', '#']
            ]),
            10
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
