mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn filter_restaurants(
    restaurants: Vec<[i32; 5]>,
    vegan_friendly: i32,
    max_price: i32,
    max_distance: i32,
) -> Vec<i32> {
    let mut res: Vec<_> = restaurants
        .into_iter()
        .filter_map(|v| {
            let [id, rating, vf, price, dist] = v;
            if vf >= vegan_friendly && price <= max_price && dist <= max_distance {
                Some([id, rating])
            } else {
                None
            }
        })
        .collect();
    res.sort_unstable_by(|a, b| b[1].cmp(&a[1]).then(b[0].cmp(&a[0])));
    res.into_iter().map(|[id, _]| id).collect()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            filter_restaurants(
                vec![
                    [1, 4, 1, 40, 10],
                    [2, 8, 0, 50, 5],
                    [3, 8, 1, 30, 4],
                    [4, 10, 0, 10, 3],
                    [5, 1, 1, 15, 1]
                ],
                1,
                50,
                10
            ),
            [3, 1, 5]
        );
        assert_eq!(
            filter_restaurants(
                vec![
                    [1, 4, 1, 40, 10],
                    [2, 8, 0, 50, 5],
                    [3, 8, 1, 30, 4],
                    [4, 10, 0, 10, 3],
                    [5, 1, 1, 15, 1]
                ],
                0,
                50,
                10
            ),
            [4, 3, 2, 1, 5]
        );
        assert_eq!(
            filter_restaurants(
                vec![
                    [1, 4, 1, 40, 10],
                    [2, 8, 0, 50, 5],
                    [3, 8, 1, 30, 4],
                    [4, 10, 0, 10, 3],
                    [5, 1, 1, 15, 1]
                ],
                0,
                30,
                3
            ),
            [4, 5]
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
