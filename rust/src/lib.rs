mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn latest_time_catch_the_bus(
    mut buses: Vec<i32>,
    mut passengers: Vec<i32>,
    capacity: i32,
) -> i32 {
    let n = passengers.len();
    buses.sort_unstable_by(|a, b| b.cmp(a));
    passengers.sort_unstable();
    let mut res = 1;
    let mut count = 0;
    let mut last = 0;
    for idx in 0..n {
        while buses.last().is_some_and(|&v| v < passengers[idx]) {
            res = buses.pop().unwrap();
        }
        if buses.is_empty() {
            break;
        }
        if last < passengers[idx] - 1 {
            res = passengers[idx] - 1;
        }
        last = passengers[idx];
        count += 1;
        if idx == n - 1
            || count == capacity
            || buses.last().is_some_and(|&v| v < passengers[1 + idx])
        {
            if count < capacity && buses.last().is_some_and(|&v| v > passengers[idx]) {
                res = *buses.last().unwrap();
            }
            buses.pop();
            count = 0;
        }
    }
    buses.first().copied().unwrap_or(res)
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
            latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2),
            16
        );
        assert_eq!(
            latest_time_catch_the_bus(vec![20, 30, 10], vec![19, 13, 26, 4, 25, 11, 21], 2),
            20
        );
    }

    #[test]
    fn test() {
        assert_eq!(latest_time_catch_the_bus(vec![3, 2], vec![2], 2), 3);
        assert_eq!(latest_time_catch_the_bus(vec![3], vec![2], 2), 3);
    }
}
