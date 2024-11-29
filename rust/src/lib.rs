mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_height_shelves(books: &[[i32; 2]], shelf_width: i32) -> i32 {
    let n = books.len();
    let mut dp = vec![i32::MAX; 1 + n];
    dp[0] = 0;
    for i1 in 1..=n {
        let mut width = 0;
        let mut height = 0;
        for i2 in (1..=i1).rev() {
            width += books[i2 - 1][0];
            if width > shelf_width {
                break;
            }
            height = height.max(books[i2 - 1][1]);
            dp[i1] = dp[i1].min(dp[i2 - 1] + height)
        }
    }
    dp[n]
    // dfs(
    //     books,
    //     shelf_width,
    //     0,
    //     shelf_width,
    //     0,
    //     &mut vec![vec![0; 1 + shelf_width as usize]; n],
    // )
}

fn dfs(
    books: &[[i32; 2]],
    shelf_width: i32,
    idx: usize,
    available: i32,
    height: i32,
    dp: &mut [Vec<i32>],
) -> i32 {
    if idx == books.len() {
        return height;
    }
    if dp[idx][available as usize] > 0 {
        return dp[idx][available as usize];
    }
    let mut res = height
        + dfs(
            books,
            shelf_width,
            1 + idx,
            shelf_width - books[idx][0],
            books[idx][1],
            dp,
        );
    if available >= books[idx][0] {
        res = res.min(dfs(
            books,
            shelf_width,
            1 + idx,
            available - books[idx][0],
            height.max(books[idx][1]),
            dp,
        ));
    }
    dp[idx][available as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            min_height_shelves(&[[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]], 4),
            6
        );
        debug_assert_eq!(min_height_shelves(&[[1, 3], [2, 4], [3, 2]], 6), 4);
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
