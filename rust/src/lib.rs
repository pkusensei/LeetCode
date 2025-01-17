mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_probability(balls: &[i32]) -> f64 {
    let n = balls.len();
    let sum = balls.iter().sum();
    let [mut box1, mut box2] = [0, 1].map(|_| vec![0; n]);
    dfs(balls, sum, &mut box1, &mut box2, 0, 0, 0) / perm(balls)
}

fn dfs(
    balls: &[i32],
    sum: i32,
    box1: &mut [i32],
    box2: &mut [i32],
    idx: usize,
    sum1: i32,
    sum2: i32,
) -> f64 {
    if sum1 > sum / 2 || sum2 > sum / 2 {
        return 0.0;
    }
    if idx >= balls.len() {
        let [c1, c2] = [&box1, &box2].map(|v| v.iter().filter(|&&v| v > 0).count());
        return if c1 == c2 {
            perm(box1) * perm(box2)
        } else {
            0.0
        };
    }
    let mut res = 0.0;
    for curr in 0..=balls[idx] {
        box1[idx] = curr;
        box2[idx] = balls[idx] - curr;
        res += dfs(
            balls,
            sum,
            box1,
            box2,
            1 + idx,
            curr + sum1,
            balls[idx] - curr + sum2,
        )
    }
    res
}

fn perm(vals: &[i32]) -> f64 {
    let mut res = 1.0;
    let mut i = 1;
    for &v in vals.iter() {
        for curr in 1..=v {
            res = res * f64::from(i) / f64::from(curr);
            i += 1;
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
        float_eq(get_probability(&[1, 1]), 1.0);
        float_eq(get_probability(&[2, 1, 1]), 0.66667);
        float_eq(get_probability(&[1, 2, 1, 2]), 0.60000);
    }

    #[test]
    fn test() {
        float_eq(get_probability(&[6, 6, 6, 6, 6, 6]), 0.5);
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
