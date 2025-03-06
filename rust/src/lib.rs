mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn halve_array(nums: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    let sum: f64 = nums.iter().map(|&v| f64::from(v)).sum();
    let mut half = sum / 2.0;
    let mut heap: BinaryHeap<_> = nums.into_iter().map(|v| Num(f64::from(v))).collect();
    let mut res = 0;
    while half >= 0.0 {
        let top = heap.pop().unwrap();
        half -= top.0 / 2.0;
        heap.push(Num(top.0 / 2.0));
        res += 1;
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Num(f64);

impl Eq for Num {}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
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
