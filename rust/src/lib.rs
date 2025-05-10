mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn minimum_cost(m: i32, n: i32, horizontal_cut: &[i32], vertical_cut: &[i32]) -> i32 {
    let [hors, vers] = [&horizontal_cut, &vertical_cut].map(|v| {
        v.iter()
            .copied()
            .enumerate()
            .sorted_unstable_by_key(|&(_i, v)| v)
            .map(|(i, v)| (1 + i, v))
            .collect_vec()
    });
    let ymax = m as usize;
    let xmax = n as usize;
    let mut memo = vec![vec![vec![vec![-1; 1 + xmax]; 1 + xmax]; 1 + ymax]; 1 + ymax];
    dfs([0, ymax], [0, xmax], hors, vers, &mut memo)
}

fn dfs(
    ys: [usize; 2],
    xs: [usize; 2],
    hors: Vec<(usize, i32)>,
    vers: Vec<(usize, i32)>,
    memo: &mut [Vec<Vec<Vec<i32>>>],
) -> i32 {
    if memo[ys[0]][ys[1]][xs[0]][xs[1]] > -1 {
        return memo[ys[0]][ys[1]][xs[0]][xs[1]];
    }
    let res = if ys[1] - ys[0] == 1 {
        if xs[1] - xs[0] == 1 {
            return 0;
        }
        let (pos, cost, v1, v2) = cut(xs, vers);
        cost + dfs(ys, [xs[0], pos], hors.clone(), v1, memo) + dfs(ys, [pos, xs[1]], hors, v2, memo)
    } else if xs[1] - xs[0] == 1 {
        let (pos, cost, v1, v2) = cut(ys, hors);
        cost + dfs([ys[0], pos], xs, v1, vers.clone(), memo) + dfs([pos, ys[1]], xs, v2, vers, memo)
    } else {
        let (pos1, cost1, v1, v2) = cut(xs, vers.clone());
        let curr = cost1
            + dfs(ys, [xs[0], pos1], hors.clone(), v1, memo)
            + dfs(ys, [pos1, xs[1]], hors.clone(), v2, memo);
        let (pos2, cost2, v11, v22) = cut(ys, hors);
        curr.min(
            cost2
                + dfs([ys[0], pos2], xs, v11, vers.clone(), memo)
                + dfs([pos2, ys[1]], xs, v22, vers, memo),
        )
    };
    memo[ys[0]][ys[1]][xs[0]][xs[1]] = res;
    res
}

fn cut(
    [c1, c2]: [usize; 2],
    mut arr: Vec<(usize, i32)>,
) -> (usize, i32, Vec<(usize, i32)>, Vec<(usize, i32)>) {
    let Some((pos, cost)) = arr.pop() else {
        unreachable!()
    };
    let v1 = arr
        .iter()
        .copied()
        .filter(|(c, _)| (c1..pos).contains(c))
        .collect_vec();
    let v2 = arr
        .iter()
        .copied()
        .filter(|(c, _)| (pos..c2).contains(c))
        .collect_vec();
    (pos, cost, v1, v2)
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
        assert_eq!(minimum_cost(3, 2, &[1, 3], &[5]), 13);
        assert_eq!(minimum_cost(2, 2, &[7], &[4]), 15);
    }

    #[test]
    fn test() {}
}
