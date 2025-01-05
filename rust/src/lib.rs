mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_city(n: i32, edges: &[[i32; 3]], distance_threshold: i32) -> i32 {
    let n = n as usize;
    let mut mat = vec![vec![1 + distance_threshold; n]; n];
    for i in 0..n {
        mat[i][i] = 0;
    }
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        mat[a][b] = e[2];
        mat[b][a] = e[2];
    }
    floyd(&mut mat, n);
    city_with_fewest_reachable(n, &mat, distance_threshold)
}

fn floyd(mat: &mut [Vec<i32>], n: usize) {
    for inter in 0..n {
        for i1 in 0..n {
            for i2 in 0..n {
                mat[i1][i2] = mat[i1][i2].min(mat[i1][inter] + mat[inter][i2]);
            }
        }
    }
}

fn city_with_fewest_reachable(n: usize, mat: &[Vec<i32>], distance_threshold: i32) -> i32 {
    let mut city = -1;
    let mut count = n;
    for i1 in 0..n {
        let mut curr = 0;
        for i2 in 0..n {
            if i1 == i2 {
                continue;
            }
            if mat[i1][i2] <= distance_threshold {
                curr += 1;
            }
        }
        if curr <= count {
            count = curr;
            city = i1 as i32;
        }
    }
    city
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            find_the_city(4, &[[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]], 4),
            3
        );
        assert_eq!(
            find_the_city(
                5,
                &[
                    [0, 1, 2],
                    [0, 4, 8],
                    [1, 2, 3],
                    [1, 4, 2],
                    [2, 3, 1],
                    [3, 4, 1]
                ],
                2
            ),
            0
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
