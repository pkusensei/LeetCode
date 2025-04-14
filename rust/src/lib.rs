mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(s1: &str, s2: &str, x: i32) -> i32 {
    let diff: Vec<i32> = s1
        .bytes()
        .zip(s2.bytes())
        .enumerate()
        .filter_map(|(i, (b1, b2))| if b1 != b2 { Some(i as i32) } else { None })
        .collect();
    let n = diff.len();
    if n & 1 == 1 {
        return -1;
    }
    dfs(&diff, f64::from(x) / 2.0, 0, &mut vec![-1.0; n]) as i32
}

fn dfs(diff: &[i32], cost: f64, idx: usize, memo: &mut [f64]) -> f64 {
    let n = diff.len();
    if idx >= n {
        return 0.0;
    }
    if idx == n - 1 {
        return cost;
    }
    if memo[idx] > -1.0 {
        return memo[idx];
    }
    let op1 = cost + dfs(diff, cost, 1 + idx, memo);
    let op2 = f64::from(diff[1 + idx] - diff[idx]) + dfs(diff, cost, 2 + idx, memo);
    memo[idx] = op1.min(op2);
    memo[idx]
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
        assert_eq!(min_operations("1100011000", "0101001010", 2), 4);
        assert_eq!(min_operations("10110", "00011", 4), -1)
    }

    #[test]
    fn test() {}
}
