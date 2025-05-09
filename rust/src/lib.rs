mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_points(mut enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
    enemy_energies.sort_unstable();
    if current_energy < enemy_energies[0] {
        return 0;
    }
    let sum = enemy_energies[1..]
        .iter()
        .map(|v| i64::from(*v))
        .sum::<i64>()
        + i64::from(current_energy);
    sum / i64::from(enemy_energies[0])
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
    fn basics() {}

    #[test]
    fn test() {}
}
