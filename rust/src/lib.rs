mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_dist_to_closest(seats: &[i32]) -> i32 {
    let n = seats.len();
    let mut prev = None;
    let mut dist = vec![n; n];
    for (i, &num) in seats.iter().enumerate() {
        match (num, prev) {
            (1, _) => {
                dist[i] = 0;
                prev = Some(i);
            }
            (0, None) => dist[i] = dist[i].min(n),
            (0, Some(v)) => dist[i] = i - v,
            _ => unreachable!(),
        }
    }
    prev = None;
    for (i, &num) in seats.iter().enumerate().rev() {
        match (num, prev) {
            (1, _) => {
                dist[i] = 0;
                prev = Some(i);
            }
            (0, None) => dist[i] = dist[i].min(n),
            (0, Some(v)) => dist[i] = dist[i].min(v - i),
            _ => unreachable!(),
        }
    }
    dist.into_iter().max().unwrap_or(1) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_dist_to_closest(&[1, 0, 0, 0, 1, 0, 1]), 2);
        debug_assert_eq!(max_dist_to_closest(&[1, 0, 0, 0]), 3);
        debug_assert_eq!(max_dist_to_closest(&[0, 1]), 1);
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
