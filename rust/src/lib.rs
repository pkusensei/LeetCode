mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    use std::cmp::Ordering;
    let mut res: Vec<i32> = vec![];
    for mut num in asteroids {
        while let Some(&v) = res.last()
            && v > 0
            && num < 0
        {
            res.pop();
            match v.abs().cmp(&num.abs()) {
                Ordering::Equal => num = 0,
                Ordering::Greater => num = v,
                Ordering::Less => (),
            }
        }
        if num != 0 {
            res.push(num);
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
