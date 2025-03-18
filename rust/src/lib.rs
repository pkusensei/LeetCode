mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
    players.sort_unstable();
    trainers.sort_unstable();
    let [mut i1, mut i2] = [0, 0];
    let mut res = 0;
    while i1 < players.len() && i2 < trainers.len() {
        while trainers.get(i2).is_some_and(|&v| v < players[i1]) {
            i2 += 1;
        }
        if i2 >= trainers.len() {
            break;
        }
        res += 1;
        i1 += 1;
        i2 += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
