mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_average_ratio(classes: &[[i32; 2]], extra_students: i32) -> f64 {
    let mut heap: std::collections::BinaryHeap<_> = classes
        .iter()
        .map(|v| Class {
            pass: v[0],
            total: v[1],
        })
        .collect();
    for _ in 0..extra_students {
        let Some(mut c) = heap.pop() else {
            continue;
        };
        c.pass += 1;
        c.total += 1;
        heap.push(c);
    }
    heap.into_iter()
        .map(|c| f64::from(c.pass) / f64::from(c.total))
        .sum::<f64>()
        / classes.len() as f64
}

#[derive(Debug, Clone, Copy)]
struct Class {
    pass: i32,
    total: i32,
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.pass * other.total == self.total * other.pass
    }
}

impl Eq for Class {}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// This is the most obvious and dirtiest way to write it
impl Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (f64::from(1 + self.pass) / f64::from(1 + self.total)
            - f64::from(self.pass) / f64::from(self.total))
        .total_cmp(
            &(f64::from(1 + other.pass) / f64::from(1 + other.total)
                - f64::from(other.pass) / f64::from(other.total)),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // float_eq(max_average_ratio(&[[1, 2], [3, 5], [2, 2]], 2), 0.78333);
        float_eq(
            max_average_ratio(&[[2, 4], [3, 9], [4, 5], [2, 10]], 4),
            0.53485,
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
            "left = {a:?}, right = {b:?}",
        );
    }
}
