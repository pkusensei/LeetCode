mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
    let mut res = [i32::MAX, i32::MIN];
    dfs(n, first_player, second_player, 0, 1, n, 1, &mut res);
    res.to_vec()
}

fn dfs(
    n: i32,
    first: i32,
    second: i32,
    mask: i32,
    mut left: i32,
    mut right: i32,
    round: i32,
    res: &mut [i32; 2],
) {
    while left < right && (mask >> left) & 1 == 1 {
        left += 1; // skip eliminated ones
    }
    while left < right && (mask >> right) & 1 == 1 {
        right -= 1;
    }
    if left >= right {
        // end of round
        dfs(n, first, second, mask, 1, n, 1 + round, res);
    } else if left == first && right == second {
        res[0] = res[0].min(round);
        res[1] = res[1].max(round);
    } else {
        // could be eliminated
        if left != first && left != second {
            dfs(
                n,
                first,
                second,
                mask | (1 << left),
                left + 1,
                right - 1,
                round,
                res,
            );
        }
        if right != first && right != second {
            dfs(
                n,
                first,
                second,
                mask | (1 << right),
                left + 1,
                right - 1,
                round,
                res,
            );
        }
    }
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(earliest_and_latest(11, 2, 4), [3, 4]);
        assert_eq!(earliest_and_latest(5, 1, 5), [1, 1]);
    }

    #[test]
    fn test() {}
}
