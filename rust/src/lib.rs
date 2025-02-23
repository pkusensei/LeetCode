mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn network_becomes_idle(edges: &[[i32; 2]], patience: &[i32]) -> i32 {
    let n = patience.len();
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [e[0], e[1]].map(|v| v as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut dists = vec![i32::MAX; n];
    dists[0] = 0;
    let mut queue = std::collections::VecDeque::from([(0, 0)]);
    while let Some((node, step)) = queue.pop_front() {
        for &next in adj[node].iter() {
            if dists[next] > step + 1 {
                dists[next] = 1 + step;
                queue.push_back((next, 1 + step));
            }
        }
    }
    dists
        .iter()
        .zip(patience.iter())
        .skip(1)
        .map(|(&d, &p)| {
            let t = 2 * d;
            let count = (t - 1) / p;
            1 + t + count * p
        })
        .max()
        .unwrap()
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
        assert_eq!(network_becomes_idle(&[[0, 1], [1, 2]], &[0, 2, 1]), 8);
        assert_eq!(
            network_becomes_idle(&[[0, 1], [0, 2], [1, 2]], &[0, 10, 10]),
            3
        );
    }

    #[test]
    fn test() {}
}
