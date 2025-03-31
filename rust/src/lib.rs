mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_total_price(n: i32, edges: &[[i32; 2]], price: &[i32], trips: &[[i32; 2]]) -> i32 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let mut freq = vec![0; n];
    for t in trips.iter() {
        count(&adj, t[1] as usize, t[0] as usize, n, &mut freq);
    }
    let [a, b] = dfs(&adj, price, &freq, 0, n);
    a.min(b)
}

fn dfs(adj: &[Vec<usize>], price: &[i32], freq: &[i32], node: usize, prev: usize) -> [i32; 2] {
    let mut full = freq[node] * price[node];
    let mut half = full / 2;
    for &next in adj[node].iter() {
        if prev != next {
            let [temp_full, temp_half] = dfs(adj, price, freq, next, node);
            full += temp_full.min(temp_half);
            half += temp_full;
        }
    }
    [full, half]
}

fn count(adj: &[Vec<usize>], goal: usize, node: usize, prev: usize, freq: &mut [i32]) -> bool {
    if node == goal {
        freq[node] += 1;
        return true;
    }
    for &next in adj[node].iter() {
        if prev != next && count(adj, goal, next, node, freq) {
            freq[node] += 1;
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
        assert_eq!(minimum_total_price(2, &[[0, 1]], &[2, 2], &[[0, 0]]), 1);
        assert_eq!(
            minimum_total_price(
                4,
                &[[0, 1], [1, 2], [1, 3]],
                &[2, 2, 10, 6],
                &[[0, 3], [2, 1], [2, 3]]
            ),
            23
        );
    }

    #[test]
    fn test() {}
}
