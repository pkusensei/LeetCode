mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_finish_time(tires: &[[i32; 2]], change_time: i32, num_laps: i32) -> i32 {
    let n = num_laps as usize;
    let change_time = i64::from(change_time);
    let mut best = vec![i64::MAX; 1 + n];
    let mut max_laps = 0;
    for tire in tires.iter() {
        let [f, r] = [0, 1].map(|i| i64::from(tire[i]));
        let mut lap_time = f;
        let mut total = lap_time;
        for lap in 1..=n {
            if lap_time >= f + change_time {
                break;
            }
            max_laps = max_laps.max(lap);
            best[lap] = best[lap].min(total);
            lap_time *= r;
            total += lap_time;
        }
    }
    dfs(n, change_time, &best, max_laps, &mut vec![i64::MAX; 1 + n]) as i32
}

fn dfs(n: usize, change_time: i64, best: &[i64], max_laps: usize, memo: &mut [i64]) -> i64 {
    if n == 0 {
        return -change_time;
    }
    if memo[n] < i64::MAX {
        return memo[n];
    }
    for lap in 1..=n.min(max_laps) {
        memo[n] =
            memo[n].min(best[lap] + change_time + dfs(n - lap, change_time, best, max_laps, memo));
    }
    memo[n]
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
        assert_eq!(minimum_finish_time(&[[2, 3], [3, 4]], 5, 4), 21);
        assert_eq!(minimum_finish_time(&[[1, 10], [2, 2], [3, 4]], 6, 5), 25);
    }

    #[test]
    fn test() {}
}
