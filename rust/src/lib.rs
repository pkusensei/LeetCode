mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
    let n = plants.len();
    let [mut i1, mut i2] = [0, n - 1];
    let [mut a, mut b] = [capacity_a, capacity_b];
    let mut res = 0;
    while i1 < i2 {
        if a >= plants[i1] {
            a -= plants[i1];
        } else {
            a = capacity_a - plants[i1];
            res += 1;
        }
        if b >= plants[i2] {
            b -= plants[i2];
        } else {
            b = capacity_b - plants[i2];
            res += 1;
        }
        i1 += 1;
        i2 -= 1;
    }
    if i1 == i2 {
        res += i32::from(a.max(b) < plants[i1]);
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
