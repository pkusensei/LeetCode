mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn path_existence_queries(
    n: i32,
    nums: &[i32],
    max_diff: i32,
    queries: &[[i32; 2]],
) -> Vec<i32> {
    let n = n as usize;
    let arr = nums
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .sorted_unstable()
        .collect_vec();
    // Convenience: Point `nums[i]` to pos in `arr`
    let mut nums_to_arr = vec![0; n];
    for (ai, (_, ni)) in arr.iter().enumerate() {
        nums_to_arr[*ni] = ai;
    }
    let h = 1 + n.ilog2() as usize;
    let jump = binary_lifting(&arr, max_diff);
    let mut res = vec![];
    for q in queries {
        let [a, b] = [0, 1].map(|i| q[i] as usize);
        if a == b {
            res.push(0);
            continue;
        }
        let aa = nums_to_arr[a].min(nums_to_arr[b]);
        let bb = nums_to_arr[a].max(nums_to_arr[b]);
        let mut node = aa;
        let mut curr = 1;
        for hh in (0..h).rev() {
            if jump[node][hh] < bb {
                node = jump[node][hh];
                curr += 1 << hh;
            }
        }
        res.push(if jump[node][0] >= bb { curr } else { -1 });
    }
    res
}

fn binary_lifting(arr: &[(i32, usize)], max_diff: i32) -> Vec<Vec<usize>> {
    let n = arr.len();
    let h = 1 + n.ilog2() as usize;
    let mut jump = vec![vec![0; h]; n];
    let mut right = 0;
    for left in 0..n {
        while right < n && arr[right].0 - arr[left].0 <= max_diff {
            right += 1;
        }
        jump[left][0] = right - 1;
    }
    for hh in 1..h {
        for node in 0..n {
            jump[node][hh] = jump[jump[node][hh - 1]][hh - 1];
        }
    }
    jump
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {}

    #[test]
    fn test() {}
}
