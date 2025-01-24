mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
    let [xmin, ymin, xmax, ymax] =
        towers
            .iter()
            .fold([i32::MAX, i32::MAX, i32::MIN, i32::MIN], |mut acc, t| {
                let x = t[0];
                let y = t[1];
                acc[0] = acc[0].min(x);
                acc[1] = acc[1].min(y);
                acc[2] = acc[2].max(x);
                acc[3] = acc[3].max(y);
                acc
            });
    let radius = f64::from(radius);
    let mut maxq = 0;
    let [mut resx, mut resy] = [0, 0];
    for x in xmin..=xmax {
        for y in ymin..=ymax {
            let mut quality = 0;
            for t in towers.iter() {
                let [tx, ty, tq] = [0, 1, 2].map(|i| t[i]);
                let d = f64::from((tx - x).pow(2) + (ty - y).pow(2)).sqrt();
                if d > radius {
                    continue;
                }
                quality += (f64::from(tq) / (1.0 + d)).floor() as i32;
            }
            if quality > maxq {
                maxq = quality;
                resx = x;
                resy = y;
            }
        }
    }
    vec![resx, resy]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

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
