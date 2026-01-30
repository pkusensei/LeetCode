mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn mirror_reflection(x: i32, y: i32) -> i32 {
    let val = lcm(x, y);
    let [a, b] = [x, y].map(|v| val / v);
    match [a & 1, b & 1] {
        [1, 1] => 1,
        [0, 1] => 0,
        _ => 2,
    }
}

// `x` is side length.
// `y` is the height where beams hits right wall for first time.
// lcm(x, y) is the length beams travels before it hits corner.
// lcm/y is even => beam hits corner on left wall => #2
// lcm/x is odd => hits #1

const fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { b } else { gcd(b % a, a) }
}

const fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
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
