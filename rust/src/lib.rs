mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_obstacle_course_at_each_position(obstacles: &[i32]) -> Vec<i32> {
    let n = obstacles.len();
    let mut sorted = Vec::with_capacity(n); // LIS
    let mut res = Vec::with_capacity(n);
    for &num in obstacles {
        let i = sorted.partition_point(|&v| v <= num);
        res.push(1 + i as i32);
        if i == sorted.len() {
            sorted.push(num);
        } else {
            sorted[i] = num;
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
        assert_eq!(
            longest_obstacle_course_at_each_position(&[1, 2, 3, 2]),
            [1, 2, 3, 3]
        );
        assert_eq!(
            longest_obstacle_course_at_each_position(&[2, 2, 1]),
            [1, 2, 1]
        );
        assert_eq!(
            longest_obstacle_course_at_each_position(&[3, 1, 5, 6, 4, 2]),
            [1, 1, 2, 3, 2, 2]
        );
    }

    #[test]
    fn test() {}
}
