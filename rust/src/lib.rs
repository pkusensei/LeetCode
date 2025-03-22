mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_fuel_cost(roads: &[[i32; 2]], seats: i32) -> i64 {
    let adj = roads.iter().fold(vec![vec![]; 100_000], |mut acc, r| {
        let [a, b] = [0, 1].map(|i| r[i] as usize);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let (f, c) = dfs(&adj, seats, 0, None);
    f - i64::from(c / seats) - i64::from(c % seats > 0)
}

// (fuel, count)
fn dfs(adj: &[Vec<usize>], seats: i32, node: usize, prev: Option<usize>) -> (i64, i32) {
    let mut fuel = 0;
    let mut count = 1;
    for &next in adj[node].iter() {
        if prev.is_none_or(|p| p != next) {
            let (f, c) = dfs(adj, seats, next, Some(node));
            fuel += f;
            count += c;
        }
    }
    fuel += i64::from(count / seats) + i64::from(count % seats > 0);
    (fuel, count)
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
        assert_eq!(minimum_fuel_cost(&[[0, 1], [0, 2], [0, 3]], 5), 3);
        assert_eq!(
            minimum_fuel_cost(&[[3, 1], [3, 2], [1, 0], [0, 4], [0, 5], [4, 6]], 2),
            7
        );
    }

    #[test]
    fn test() {}
}
