mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mice_and_cheese(reward1: &[i32], reward2: &[i32], k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut heap: BinaryHeap<_> = reward1
        .iter()
        .zip(reward2.iter())
        .map(|(a, b)| a - b)
        .collect();
    let mut res: i32 = reward2.iter().sum::<i32>();
    for _ in 0..k {
        let Some(diff) = heap.pop() else {
            break;
        };
        res += diff
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
        assert_eq!(mice_and_cheese(&[1, 1, 3, 4], &[4, 4, 1, 1], 2), 15);
    }

    #[test]
    fn test() {}
}
