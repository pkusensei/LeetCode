mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_build_rooms(prev_room: &[i32]) -> i32 {
    let n = prev_room.len();
    let adj =
        prev_room
            .iter()
            .enumerate()
            .skip(1)
            .fold(vec![vec![]; n], |mut acc, (prev, &node)| {
                acc[node as usize].push(prev);
                acc
            });
    let mut size = vec![0; n];
    dfs(&adj, &mut size, 0);
    let prod = size.iter().fold(1, |acc, &num| acc * i64::from(num) % MOD);
    let fact = (1..=n).fold(1, |acc, num| acc * num as i64 % MOD);
    let inverse = mod_pow(prod, MOD - 2, MOD);
    (inverse * fact % MOD) as i32
}

const MOD: i64 = 1_000_000_007;

// size of substree
fn dfs(adj: &[Vec<usize>], size: &mut [i32], curr: usize) -> i32 {
    let res = 1 + adj[curr]
        .iter()
        .map(|&next| dfs(adj, size, next))
        .sum::<i32>();
    size[curr] = res;
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
        assert_eq!(ways_to_build_rooms(&[-1, 0, 1]), 1);
        assert_eq!(ways_to_build_rooms(&[-1, 0, 0, 1, 2]), 6);
    }

    #[test]
    fn test() {}
}
