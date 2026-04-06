mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    const D: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let obs: HashSet<_> = obstacles.iter().map(|v| [v[0], v[1]]).collect();
    let mut dir = 0;
    let [mut x, mut y] = [0, 0];
    let mut res = 0;
    for k in commands {
        match k {
            -2 => dir = (dir + 3) % 4,
            -1 => dir = (dir + 1) % 4,
            _ => {
                let [dx, dy] = D[dir];
                for _ in 0..k {
                    let nx = x + dx;
                    let ny = y + dy;
                    if obs.contains(&[nx, ny]) {
                        break;
                    }
                    x = nx;
                    y = ny;
                }
                res = res.max(x.pow(2) + y.pow(2));
            }
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
