mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_coprimes(nums: &[i32], edges: &[[i32; 2]]) -> Vec<i32> {
    let n = nums.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        acc[e[0] as usize].push(e[1]);
        acc[e[1] as usize].push(e[0]);
        acc
    });
    let mut res = vec![-1; n];
    let mut path = vec![vec![]; 51];
    dfs(nums, &adj, 0, -1, 0, &mut path, &mut res);
    res
}

fn dfs(
    nums: &[i32],
    adj: &[Vec<i32>],
    node: i32,
    prev: i32,
    depth: i32,
    path: &mut [Vec<[i32; 2]>],
    res: &mut [i32],
) {
    let mut closest = -1;
    let mut max_depth = -1;
    for num in 1..=50 {
        let Some(&[idx, depth]) = path[num as usize].last() else {
            continue;
        };
        if gcd(num, nums[node as usize]) == 1 && depth > max_depth {
            closest = idx;
            max_depth = depth;
        }
    }
    res[node as usize] = closest;
    path[nums[node as usize] as usize].push([node as i32, depth]);
    for &next in adj[node as usize].iter() {
        if next == prev {
            continue;
        }
        dfs(nums, adj, next, node, 1 + depth, path, res);
    }
    path[nums[node as usize] as usize].pop();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            get_coprimes(&[2, 3, 3, 2], &[[0, 1], [1, 2], [1, 3]]),
            [-1, 0, 0, 1]
        );
        assert_eq!(
            get_coprimes(
                &[5, 6, 10, 2, 3, 6, 15],
                &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]
            ),
            [-1, 0, -1, 0, 0, 0, -1]
        );
    }

    #[test]
    fn test() {}
}
