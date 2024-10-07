mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (rows, cols) = get_dimensions(&img);
    let mut res = img.clone();
    for y in 0..rows {
        for x in 0..cols {
            let sum: f64 = around(x as i32, y as i32)
                .chain(std::iter::once((x, y)))
                .filter_map(|(x, y)| img.get(y).map(|r| r.get(x)))
                .map(|opt| opt.map(|&n| f64::from(n)).unwrap_or(0.0))
                .sum();
            let count = around(x as i32, y as i32)
                .filter(|&(x, y)| x < cols && y < rows)
                .count() as f64
                + 1.0;
            res[y][x] = (sum / count).trunc() as i32;
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
            image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
        );
        debug_assert_eq!(
            image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            [[137, 141, 137], [141, 138, 141], [137, 141, 137]]
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
}
