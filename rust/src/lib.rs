mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn component_value(nums: &[i32], edges: &[[i32; 2]]) -> i32 {
    let n = nums.len();
    let mut adj = vec![vec![]; n];
    let mut degs = vec![0; n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        adj[a].push(b);
        adj[b].push(a);
        degs[a] += 1;
        degs[b] += 1;
    }
    let [max, sum] = nums
        .iter()
        .fold([0, 0], |[max, sum], &num| [max.max(num), sum + num]);
    for div in (max..=sum).filter(|&v| sum % v == 0) {
        if bfs(&adj, nums.to_vec(), degs.clone(), div) {
            return sum / div - 1;
        }
    }
    0
}

fn bfs(adj: &[Vec<usize>], mut nums: Vec<i32>, mut degs: Vec<i32>, target: i32) -> bool {
    let mut queue: VecDeque<_> = degs
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == 1 { Some(i) } else { None })
        .collect();
    while let Some(node) = queue.pop_front() {
        for &next in adj[node].iter() {
            if nums[node] > target {
                return false;
            }
            nums[next] += if nums[node] < target { nums[node] } else { 0 };
            degs[next] -= 1;
            if degs[next] == 1 {
                queue.push_back(next);
            }
        }
    }
    true
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
            component_value(&[6, 2, 2, 2, 6], &[[0, 1], [1, 2], [1, 3], [3, 4]]),
            2
        );
    }

    #[test]
    fn test() {}
}
