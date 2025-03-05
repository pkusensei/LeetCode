mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut grid = vec![vec![false; n]; n];
    for d in dig {
        let [r, c] = d[..] else { unreachable!() };
        grid[r as usize][c as usize] = true;
    }
    artifacts
        .iter()
        .filter(|a| {
            let [r1, c1, r2, c2] = a[..] else {
                unreachable!()
            };
            for r in r1..=r2 {
                for c in c1..=c2 {
                    if !grid[r as usize][c as usize] {
                        return false;
                    }
                }
            }
            true
        })
        .count() as _
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
