mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    let mut max_d = 0.0;
    let mut res = 0;
    for v in dimensions {
        let [a, b] = v[..] else { unreachable!() };
        let d = f64::from(a.pow(2) + b.pow(2)).sqrt();
        if d > max_d {
            max_d = d;
            res = a * b;
        } else if d == max_d {
            res = res.max(a * b);
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
