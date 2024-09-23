mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
struct Solution {
    x: f64,
    y: f64,
    radius: f64,
}

impl Solution {
    fn new(radius: f64, x: f64, y: f64) -> Self {
        Self { x, y, radius }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let angle = rng.gen::<f64>() * 2f64 * std::f64::consts::PI;
        let radius = self.radius * rng.gen::<f64>().sqrt();
        vec![self.x + radius * angle.cos(), self.y + radius * angle.sin()]
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
