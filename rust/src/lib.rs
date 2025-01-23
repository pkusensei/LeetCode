mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn is_printable(target_grid: &[&[i32]]) -> bool {
    let mut bounds: HashMap<i32, [usize; 4]> = HashMap::new();
    for (r, row) in target_grid.iter().enumerate() {
        for (c, &color) in row.iter().enumerate() {
            if let Some(v) = bounds.get_mut(&color) {
                let min_row = v[0].min(r);
                let min_col = v[1].min(c);
                let max_row = v[2].max(r);
                let max_col = v[3].max(c);
                *v = [min_row, min_col, max_row, max_col];
            } else {
                bounds.insert(color, [r, c, r, c]);
            }
        }
    }
    let mut empty = HashSet::new();
    while let Some(color) = find_rect(target_grid, &bounds, &empty) {
        let Some([minr, minc, maxr, maxc]) = bounds.remove(&color) else {
            return false;
        };
        // remove this color from board
        for r in minr..=maxr {
            for c in minc..=maxc {
                empty.insert([r, c]);
            }
        }
    }
    let [rows, cols] = get_dimensions(target_grid);
    rows * cols == empty.len()
}

fn find_rect(
    grid: &[&[i32]],
    bounds: &HashMap<i32, [usize; 4]>,
    empty: &HashSet<[usize; 2]>,
) -> Option<i32> {
    'outer: for (&color, &[minr, minc, maxr, maxc]) in bounds.iter() {
        for r in minr..=maxr {
            for c in minc..=maxc {
                if grid[r][c] != color && !empty.contains(&[r, c]) {
                    continue 'outer; // current color is not rectangle yet
                }
            }
        }
        return Some(color);
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(is_printable(&[
            &[1, 1, 1, 1],
            &[1, 2, 2, 1],
            &[1, 2, 2, 1],
            &[1, 1, 1, 1]
        ]));
        assert!(is_printable(&[
            &[1, 1, 1, 1],
            &[1, 1, 3, 3],
            &[1, 1, 3, 4],
            &[5, 5, 1, 4]
        ]));
        assert!(!is_printable(&[&[1, 2, 1], &[2, 1, 2], &[1, 2, 1]]));
    }

    #[test]
    fn test() {
        assert!(!is_printable(&[
            &[5, 1, 5, 3, 5],
            &[4, 4, 4, 3, 4],
            &[5, 1, 5, 3, 5],
            &[2, 1, 2, 2, 2],
            &[5, 1, 5, 3, 5]
        ]));
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
