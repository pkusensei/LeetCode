mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
    let mut res = -1;
    let [tx, ty] = target[..] else { unreachable!() };
    let mut dist = 1000;
    for (i, drone) in drones.iter().enumerate() {
        let [dx, dy, r] = drone[..] else {
            unreachable!()
        };
        let curr = (dx - tx).abs() + (dy - ty).abs();
        if curr <= r {
            if res == -1 || dist > curr {
                res = i as i32;
                dist = curr;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
