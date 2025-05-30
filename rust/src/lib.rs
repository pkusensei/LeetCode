mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn bottom_up(s: &str) -> i32 {
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let max = *freq.iter().max().unwrap_or(&1);
    let mut res = s.len() as i32;
    for target in 0..=max {
        // min cost so far, assuming change-to-next is allowed
        let mut curr_cost = 0;
        // total cost with only deletions
        let mut del_cost = s.len() as i32;
        // Excess chars deleted in previous round
        // potentially for reuse in next round
        let mut save = 0;
        for &val in &freq {
            if val >= target {
                curr_cost += val - target;
                del_cost = curr_cost;
                save = val - target;
            } else {
                // To fill the diff (target-val)
                // 1) pure addition
                // 2) reuse save from previous round
                let diff = target - val;
                let increment = (curr_cost + diff).min(del_cost + (diff - save).max(0));
                // delete all
                let remove = curr_cost + val;
                curr_cost = increment.min(remove);
                del_cost = remove;
                // Potentially delete all and reuse
                save = val;
            }
        }
        res = res.min(curr_cost);
    }
    res
}

pub fn make_string_good(s: &str) -> i32 {
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let max = *freq.iter().max().unwrap_or(&1);
    let mut res = s.len() as i32;
    for target in 0..=max {
        res = res.min(dfs(&freq, target, 0, 0, &mut HashMap::new()));
    }
    res
}

fn dfs(
    freq: &[i32; 26],
    target: i32,
    idx: usize,
    del: i32,
    memo: &mut HashMap<(usize, i32), i32>,
) -> i32 {
    if idx >= 26 {
        return 0;
    }
    if let Some(&v) = memo.get(&(idx, del)) {
        return v;
    }
    let val = freq[idx];
    let res = if val > target {
        val - target + dfs(freq, target, 1 + idx, val - target, memo)
    } else {
        // delete all [idx]
        let remove = val + dfs(freq, target, 1 + idx, val, memo);
        // Possible increases using op3, i.e decrease [idx-1] and increase [idx]
        // The remaining increment is done by (target-val-op3)
        let op3 = del.min(target - val);
        let increment = target - val - op3 + dfs(freq, target, 1 + idx, 0, memo);
        remove.min(increment)
    };
    memo.insert((idx, del), res);
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
        assert_eq!(make_string_good("acab"), 1);
        assert_eq!(make_string_good("wddw"), 0);
        assert_eq!(make_string_good("aaabc"), 2);

        assert_eq!(bottom_up("acab"), 1);
        assert_eq!(bottom_up("wddw"), 0);
        assert_eq!(bottom_up("aaabc"), 2);
    }

    #[test]
    fn test() {}
}
