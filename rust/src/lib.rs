mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_fruits(fruits: &[[i32; 2]], start_pos: i32, k: i32) -> i32 {
    let [start, k] = [start_pos, k].map(|v| v as usize);
    let n = (fruits.last().unwrap()[0] as usize).max(start) + 1;
    let mut prefix = vec![0; n];
    for fr in fruits.iter() {
        prefix[fr[0] as usize] = fr[1];
    }
    for i in 1..n {
        prefix[i] += prefix[i - 1];
    }
    let walk_left = prefix[start] - start.checked_sub(1 + k).map(|i| prefix[i]).unwrap_or(0);
    let walk_right = prefix[(start + k).min(n - 1)] - if start > 0 { prefix[start - 1] } else { 0 };
    let mut res = walk_left.max(walk_right);
    for v in (1..=k / 2).map(|x| {
        let a = start.saturating_sub(x);
        let b = (start + k - 2 * x).min(n - 1);
        prefix[b] - if a > 0 { prefix[a - 1] } else { 0 }
    }) {
        res = res.max(v)
    }
    for v in (1..=k / 2).map(|x| {
        let a = start.saturating_sub(k - 2 * x);
        let b = (start + x).min(n - 1);
        prefix[b] - if a > 0 { prefix[a - 1] } else { 0 }
    }) {
        res = res.max(v)
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
        assert_eq!(max_total_fruits(&[[2, 8], [6, 3], [8, 6]], 5, 4), 9);
        assert_eq!(
            max_total_fruits(&[[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]], 5, 4),
            14
        );
        assert_eq!(max_total_fruits(&[[0, 3], [6, 4], [8, 5]], 3, 2), 0);
    }

    #[test]
    fn test() {
        assert_eq!(max_total_fruits(&[[0, 10000]], 200000, 200000), 10000);
        assert_eq!(
            max_total_fruits(
                &[[1, 8], [6, 5], [8, 6], [10, 1], [11, 7], [12, 6], [20, 3]],
                7,
                3
            ),
            11
        );
        // 8  5  6  1  7  6  3
        // 1  6  8  10 11 12 20
    }
}
