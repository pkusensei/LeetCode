mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    variables
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            let [a, b, c, m] = v[..] else { unreachable!() };
            let x = mod_pow(a.into(), b.into(), 10);
            let y = mod_pow(x.into(), c.into(), m.into());
            if y == i64::from(target) {
                Some(i as i32)
            } else {
                None
            }
        })
        .collect()
}

const fn mod_pow(base: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(base * base % m, exp >> 1, m)
    } else {
        base * mod_pow(base * base % m, exp >> 1, m) % m
    }
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
