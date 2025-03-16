mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_sequence(rolls: &[i32], k: i32) -> i32 {
    let mut len = 1;
    let mut set = std::collections::HashSet::new();
    for &num in rolls.iter() {
        set.insert(num);
        // For subseq len==1, find minimum index that all k nums are seen
        // Then for len==2, start from current min index,
        // All k nums have to be found again.
        if set.len() == k as usize {
            set.clear();
            len += 1;
        }
    }
    len
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
        assert_eq!(shortest_sequence(&[4, 2, 1, 2, 3, 3, 2, 4, 1], 4), 3);
        assert_eq!(shortest_sequence(&[1, 1, 2, 2], 2), 2);
        assert_eq!(shortest_sequence(&[1, 1, 3, 2, 2, 2, 3, 3], 4), 1);
    }

    #[test]
    fn test() {}
}
