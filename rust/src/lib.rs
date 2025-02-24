mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time(n: i32, relations: &[[i32; 2]], time: &[i32]) -> i32 {
    let n = n as usize;
    let mut indegs = vec![0; n];
    let mut adj = vec![vec![]; n];
    for e in relations.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        adj[a].push(b);
        indegs[b] += 1;
    }
    let mut dp = vec![0; n];
    // (node, time)
    let mut queue = std::collections::VecDeque::new();
    for (node, &deg) in indegs.iter().enumerate() {
        if deg == 0 {
            dp[node] = time[node];
            queue.push_back(node);
        }
    }
    let mut res = 0;
    while let Some(node) = queue.pop_front() {
        for &next in adj[node].iter() {
            indegs[next] -= 1;
            dp[next] = dp[next].max(time[next] + dp[node]);
            if indegs[next] == 0 {
                queue.push_back(next);
            }
        }
        res = res.max(dp[node])
    }
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
        assert_eq!(minimum_time(3, &[[1, 3], [2, 3]], &[3, 2, 5]), 8);
        assert_eq!(
            minimum_time(
                5,
                &[[1, 5], [2, 5], [3, 5], [3, 4], [4, 5]],
                &[1, 2, 3, 4, 5]
            ),
            12
        );
    }

    #[test]
    fn test() {}
}
