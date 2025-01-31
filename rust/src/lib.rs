mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
    box_types.sort_unstable_by_key(|b| std::cmp::Reverse(b[1]));
    let mut res = 0;
    for bt in box_types {
        if truck_size <= 0 {
            break;
        }
        let [b, u] = bt[..] else {
            continue;
        };
        res += b.min(truck_size) * u;
        truck_size -= b.min(truck_size);
    }
    res
}

#[cfg(test)]
mod tests {

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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
