mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_boxes(n: i32) -> i32 {
    let mut curr = 0;
    let mut level = 0;
    let mut area = 0;
    while curr < n {
        level += 1;
        area += level;
        curr += area;
    }
    if curr == n {
        return area;
    }
    curr -= area;
    area -= level;
    level = 0;
    while curr < n {
        level += 1;
        curr += level;
    }
    area + level
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
        assert_eq!(minimum_boxes(3), 3);
        assert_eq!(minimum_boxes(4), 3);
        assert_eq!(minimum_boxes(10), 6);
    }

    #[test]
    fn test() {
        assert_eq!(minimum_boxes(15), 9);
    }
}
