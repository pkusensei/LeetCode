mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_happy_groups(batch_size: i32, groups: &[i32]) -> i32 {
    let batch = batch_size as usize;
    let mut res = 0;
    let mut freqs = vec![0; batch];
    for g in groups.iter() {
        let rem = g % batch_size;
        if rem == 0 {
            res += 1;
        } else if freqs[batch - rem as usize] > 0 {
            freqs[batch - rem as usize] -= 1; // greedy
            res += 1;
        } else {
            freqs[rem as usize] += 1;
        }
    }
    res + dfs(&mut freqs, 0, &mut HashMap::new())
}

fn dfs(freqs: &mut [i32], last: usize, memo: &mut HashMap<Vec<i32>, i32>) -> i32 {
    if let Some(&v) = memo.get(freqs) {
        return v;
    }
    let batch = freqs.len();
    let mut res = 0;
    for sz in 1..batch {
        if freqs[sz] > 0 {
            freqs[sz] -= 1;
            res = res.max(i32::from(last == 0) + dfs(freqs, (last + sz) % batch, memo));
            freqs[sz] += 1;
        }
    }
    memo.insert(freqs.to_vec(), res);
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
        assert_eq!(max_happy_groups(3, &[1, 2, 3, 4, 5, 6]), 4);
        assert_eq!(max_happy_groups(4, &[1, 3, 2, 5, 2, 2, 1, 6]), 4);
    }

    #[test]
    fn test() {}
}
