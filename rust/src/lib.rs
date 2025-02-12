mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_wasted_space(packages: Vec<i32>, mut boxes: Vec<Vec<i32>>) -> i32 {
    const MAX: usize = 100_000;
    let mut prefix = packages.iter().fold(vec![0; 1 + MAX], |mut acc, &p| {
        acc[p as usize] += 1;
        acc
    });
    for i in 1..=MAX {
        prefix[i] += prefix[i - 1];
    }
    let sum: i64 = packages.iter().map(|&v| i64::from(v)).sum();
    let mut res = i64::MAX;
    for b in boxes.iter_mut() {
        if let Some(v) = fit(&prefix, b) {
            res = res.min(v)
        }
    }
    if res == i64::MAX {
        -1
    } else {
        ((res - sum) % 1_000_000_007) as _
    }
}

fn fit(prefix: &[i32], boxes: &mut [i32]) -> Option<i64> {
    boxes.sort_unstable();
    let mut count = *prefix.last().unwrap();
    let mut res = 0;
    for (idx, &right) in boxes.iter().enumerate() {
        let left = if idx == 0 { 0 } else { boxes[idx - 1] };
        let window_count = prefix[right as usize] - prefix[left as usize];
        if window_count > 0 {
            res += i64::from(right) * i64::from(window_count);
            count -= window_count;
        }
    }
    if count == 0 {
        Some(res)
    } else {
        None
    }
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
            min_wasted_space(vec![2, 3, 5], vec![vec![4, 8], vec![2, 8]]),
            6
        );
        assert_eq!(
            min_wasted_space(vec![2, 3, 5], vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
            -1
        );
        assert_eq!(
            min_wasted_space(
                vec![3, 5, 8, 10, 11, 12],
                vec![vec![12], vec![11, 9], vec![10, 5, 14]]
            ),
            9
        );
    }

    #[test]
    fn test() {}
}
