mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn recover_array(_n: i32, sums: &mut [i32]) -> Vec<i32> {
    sums.sort_unstable();
    dfs(sums)
}

fn dfs(sums: &[i32]) -> Vec<i32> {
    if sums.len() == 1 {
        return vec![]; // sums is [0]
    }
    let diff = sums[1] - sums[0];
    let mut count = sums.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    // exc+diff==inc
    let [mut excluding_diff, mut including_diff] = [vec![], vec![]];
    let mut pick_including_diff = false;
    for &exc in sums.iter() {
        let inc = exc + diff;
        if count.get(&exc).is_some_and(|&v| v > 0) {
            count.entry(exc).and_modify(|v| *v -= 1);
            count.entry(inc).and_modify(|v| *v -= 1);
            excluding_diff.push(exc);
            including_diff.push(inc);
            if inc == 0 {
                pick_including_diff = true; // split with 0 must be worked on
            }
        }
    }
    let mut res = if pick_including_diff {
        dfs(&including_diff)
    } else {
        dfs(&excluding_diff)
    };
    res.push(if pick_including_diff { -diff } else { diff });
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
        sort_eq!(
            recover_array(3, &mut [-3, -2, -1, 0, 0, 1, 2, 3]),
            [-1, -2, 3]
        );
        sort_eq!(recover_array(2, &mut [0, 0, 0, 0]), [0, 0]);
        sort_eq!(
            recover_array(4, &mut [0, 0, 5, 5, 4, -1, 4, 9, 9, -1, 4, 3, 4, 8, 3, 8]),
            [0, -1, 4, 5]
        );
    }

    #[test]
    fn test() {}
}
