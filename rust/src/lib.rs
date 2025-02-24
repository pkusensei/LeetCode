mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn most_profitable_path(edges: &[[i32; 2]], bob: i32, amount: &[i32]) -> i32 {
    let n = amount.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut b_steps = vec![i32::MAX; n];
    dfs_b(&adj, bob as _, None, 0, &mut b_steps);
    dfs_a(&adj, amount, &b_steps, 0, None, 0)
}

fn dfs_a(
    adj: &[Vec<usize>],
    amount: &[i32],
    b_steps: &[i32],
    node: usize,
    prev: Option<usize>,
    curr_step: i32,
) -> i32 {
    let curr_score = match curr_step.cmp(&b_steps[node]) {
        std::cmp::Ordering::Less => amount[node],
        std::cmp::Ordering::Equal => amount[node] / 2,
        std::cmp::Ordering::Greater => 0,
    };
    let mut res = i32::MIN;
    for &next in adj[node].iter() {
        if prev.is_some_and(|p| p == next) {
            continue;
        }
        res = res.max(dfs_a(adj, amount, b_steps, next, Some(node), 1 + curr_step));
    }
    if res == i32::MIN {
        curr_score
    } else {
        res + curr_score
    }
}

fn dfs_b(
    adj: &[Vec<usize>],
    node: usize,
    prev: Option<usize>,
    curr_step: i32,
    b_steps: &mut [i32],
) -> bool {
    if b_steps[node] < i32::MAX {
        return false;
    }
    if node == 0 && b_steps[0] == i32::MAX {
        b_steps[0] = curr_step;
        return true;
    }
    for &next in adj[node].iter() {
        if prev.is_some_and(|p| p == next) {
            continue;
        }
        if dfs_b(adj, next, Some(node), 1 + curr_step, b_steps) {
            b_steps[node] = curr_step;
            return true;
        }
    }
    false
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
            most_profitable_path(&[[0, 1], [1, 2], [1, 3], [3, 4]], 3, &[-2, 4, 2, -4, 6]),
            6
        );
        assert_eq!(most_profitable_path(&[[0, 1]], 1, &[-7280, 2350]), -7280);
    }

    #[test]
    fn test() {}
}
