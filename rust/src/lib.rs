mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    dfs(
        n,
        first_player,
        n + 1 - second_player,
        1,
        &mut min,
        &mut max,
    );
    vec![min, max]
}

fn dfs(n: i32, mut left: i32, mut right: i32, round: i32, min: &mut i32, max: &mut i32) {
    if left == right {
        *min = (*min).min(round);
        *max = (*max).max(round);
        return;
    }
    if left > right {
        std::mem::swap(&mut left, &mut right);
    }
    for new_left in 1..=left {
        let mut new_right = left + 1 - new_left;
        while new_left + new_right <= right.min((1 + n) / 2) {
            if left + right - new_left - new_right <= n / 2 {
                dfs((1 + n) / 2, new_left, new_right, 1 + round, min, max);
            }
            new_right += 1;
        }
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
        assert_eq!(earliest_and_latest(11, 2, 4), [3, 4]);
        assert_eq!(earliest_and_latest(5, 1, 5), [1, 1]);
    }

    #[test]
    fn test() {}
}
