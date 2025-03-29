mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_reach_target(target: i32, types: &[[i32; 2]]) -> i32 {
    let target = target as usize;
    let mut dp = vec![0; 1 + target];
    dp[0] = 1;
    for t in types.iter() {
        let [count, mark] = [t[0], t[1]];
        let mut curr = vec![0; 1 + target];
        for c in 0..=count {
            for (prev_s, v) in dp.iter().enumerate() {
                let score = prev_s + (c * mark) as usize;
                if score <= target {
                    curr[score] += v;
                    curr[score] %= 1_000_000_007;
                }
            }
        }
        dp = curr;
    }
    dp[target]
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
        assert_eq!(ways_to_reach_target(6, &[[6, 1], [3, 2], [2, 3]]), 7);
        assert_eq!(ways_to_reach_target(5, &[[50, 1], [50, 2], [50, 5]]), 4);
    }

    #[test]
    fn test() {}
}
