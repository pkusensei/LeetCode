mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_number_of_hours(
    initial_energy: i32,
    initial_experience: i32,
    energy: Vec<i32>,
    experience: Vec<i32>,
) -> i32 {
    let mut res = (energy.iter().sum::<i32>() + 1 - initial_energy).max(0);
    let mut exp = initial_experience;
    for &e in experience.iter() {
        if exp > e {
            exp += e;
        } else {
            let diff = e + 1 - exp;
            exp += diff + e;
            res += diff;
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
