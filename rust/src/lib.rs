mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check(mut nums: Vec<i32>) -> bool {
    let mut it = nums
        .windows(2)
        .enumerate()
        .filter_map(|(i, w)| if w[0] > w[1] { Some(i) } else { None });
    match (it.next(), it.next()) {
        (None, None) => true,
        (Some(idx), None) => {
            nums.rotate_left(1 + idx);
            nums.windows(2).all(|w| w[0] <= w[1])
        }
        _ => false,
    }
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
    fn basics() {
        assert!(check(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn test() {}
}
