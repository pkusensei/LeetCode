mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_choose(groups: &[&[i32]], nums: &[i32]) -> bool {
    let mut arr = nums;
    for group in groups.iter() {
        let n = group.len();
        let Some(i) = arr
            .windows(n)
            .enumerate()
            .find_map(|(i, w)| if w == *group { Some(i) } else { None })
        else {
            return false;
        };
        arr = &arr[i + n..];
    }
    true
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
        assert!(can_choose(
            &[&[1, -1, -1], &[3, -2, 0]],
            &[1, -1, 0, 1, -1, -1, 3, -2, 0]
        ));
        assert!(!can_choose(
            &[&[10, -2], &[1, 2, 3, 4]],
            &[1, 2, 3, 4, 10, -2]
        ));
        assert!(!can_choose(
            &[&[1, 2, 3], &[3, 4]],
            &[7, 7, 1, 2, 3, 4, 7, 7]
        ));
    }

    #[test]
    fn test() {}
}
