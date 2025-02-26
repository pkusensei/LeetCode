mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn watering_plants(plants: &[i32], capacity: i32) -> i32 {
    let mut curr = capacity;
    let mut res = 0;
    for (i, &p) in (0..).zip(plants.iter()) {
        if curr >= p {
            curr -= p;
            res += 1;
        } else {
            curr = capacity - p;
            res += 2 * i + 1;
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
    fn basics() {
        assert_eq!(watering_plants(&[2, 2, 3, 3], 5), 14);
    }

    #[test]
    fn test() {}
}
